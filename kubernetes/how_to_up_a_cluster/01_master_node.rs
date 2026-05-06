INSTAL VM SOFTWARE
// qemu-kvm - hypervisor, to run vm an native OS
// libvirt-deamon-system - make my laptop into host for vms
// virtinst - cli to create vms
sudo apt update
sudo apt install -y qemu-kvm libvirt-daemon-system libvirt-clients bridge-utils virtinst

ADD TO GROOPS
sudo usermod -aG libvirt $USER
sudo usermod -aG kvm $USER
// reboot

CHECK
// shows empty list of vms
virsh list --all

CHECK NETWORKS
// show networks to comunicate between vms
sudo virsh net-list --all
// default - is the name of the network that creates by default

// if default network is inactive, start it
sudo virsh net-start default
sudo virsh net-autostart default

DOWNLOAD UBUNTU SERVER
// go to releases.ubuntu.com
// go to version folder and copy the link
wget -c https://releases.ubuntu.com/22.04/ubuntu-22.04.5-live-server-amd64.iso

CREATE K8S MASTER
// move iso to
sudo mv /home/ang/kvm/iso/ubuntu-22.04.5-live-server-amd64.iso /var/lib/libvirt/images/

// add permissions
sudo chown libvirt-qemu:kvm /var/lib/libvirt/images/ubuntu-22.04.5-live-server-amd64.iso

// create vm
virt-install \
  --name k8s-master \
  --ram 2048 \
  --vcpus 2 \
  --disk path=/var/lib/libvirt/images/k8s-master.qcow2,size=20,bus=virtio \
  --os-variant ubuntu22.04 \
  --network network=default,model=virtio \
  --graphics vnc,listen=0.0.0.0 \
  --console pty,target_type=serial \
  --cdrom /var/lib/libvirt/images/ubuntu-22.04.5-live-server-amd64.iso

// or if have name conflict, delet the name from virsh
virsh undefine k8s-master

GO TO VIRT_MANAGER
// go to virt manager and install ubuntu
// do not forget mark install ssh-deamon
virt-manager

// after install reboot sysete

// if blinging _ just press enter and login

FIRST CONNECT
// get interfaces
ip a

// copy ip after enp1s0
// fe
192.168.122.191
// now i can close virt-manager, and connect to vm via ssh with my terminal
ssh ang@192.168.122.19191
// and type the password

VM CONFIG FOR K8S MASTER
// turn off swap - required for kubernetes
sudo swapoff -a // k8s wants to rule memory by self, turn off swap in this session
sudo sed -i '/swap/s/^/#/' /etc/fstab // turn off swap for all sessions, it finds line with swap, and comment it with #


// create a file k8n.conf and add to it two rows
// add it to start it next time
cat <<EOF | sudo tee /etc/modules-load.d/k8s.conf
overlay // need for containers will be able to use OverlayFS (multilayer images) - this is base for contanerd and docker
br_netfilter // allows the network bridge to pass traffic through the firewall(iptables), without it k8s cant manage network rules for its pods
EOF

// start it now
sudo modprobe overlay // turn it on
sudo modprobe br_netfilter // turn it on

// check runs it or not
lsmod | grep -e overlay -e br_netfilter

// configuring sysctl to alow master works like router for containers
// by creating config file
cat <<EOF | sudo tee /etc/sysctl.d/k8s.conf
net.bridge.bridge-nf-call-iptables  = 1 // forces traffic that goes through v briges to go though iptables rules (firewall)
net.bridge.bridge-nf-call-ip6tables = 1 // the same but with ip6
net.ipv4.ip_forward                 = 1 // allows OS to forward packages that are not intended for itself, but for other vms
EOF

// say to kernel to rewrite all config files
sudo sysctl --system

INSTALL CONTAINERD ON MASTER
// conteinerd is a gold standart for K8S instead of heavyweigh Docker
sudo apt update
sudo apt install -y containerd

// create a standart config
sudo mkdir -p /etc/containerd
containerd config default | sudo tee /etc/containerd/config.toml

// corfig a systemd
// k8s and containerd shoul use the same resource manager Cgroup, or cluster cant start
// this command find SystemdCgroup = false and change it to true
sudo sed -i 's/SystemdCgroup \= false/SystemdCgroup \= true/g' /etc/containerd/config.toml

// reboot and check
sudo systemctl restart containerd
sudo systemctl enable containerd
systemctl status containerd


INSTALL KUBERNETES
// that makes this vm into cluster node
// yandex mirror
https://mirror.yandex.ru/mirrors/pkgs.k8s.io/core/stable/
// go into version i need fe 1.34
// go into deb/arm46/
// downoload latest kubeadm, kubectl, kubelet, kubernetes-cni, cri-tools

// move to vm
scp *.deb ang@192.168.122.191:~/

// install
sudo dpkg -i *.deb

INIT MASTER NODE
sudo kubeadm init --pod-network-cidr=10.244.0.0/16

// to use cluster with my user
mkdir -p $HOME/.kube
sudo cp -i /etc/kubernetes/admin.conf $HOME/.kube/config
sudo chown $(id -u):$(id -g) $HOME/.kube/config

// install CNI
kubectl apply -f https://raw.githubusercontent.com/coreos/flannel/master/Documentation/kube-flannel.yml

// verify instalation
// list of pods, there will be all default kuber pods
kubectl get pods -A
// list of nodes, there will be only one master node
kubectl get nodes

CONFIG STATIC IP FOR MASTER
// need to restart vm on same ip address

// run on laptop not on vm
// get vm mac
virsh dumpxml k8s-master | grep "mac address"
//<mac address='52:54:00:22:46:f1'/>

// configure virsh net
sudo virsh net-edit default
// choose vim and add mac to dhcp section
<dhcp>
    <range start='192.168.122.2' end='192.168.122.254'/>
    <host mac='52:54:00:22:46:f1' name='k8s-master' ip='192.168.122.191'/>
</dhcp>

// restart network
sudo virsh net-destroy default
sudo virsh net-start default
