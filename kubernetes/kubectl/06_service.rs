# SERVICES
- ClusterIP - gives ip and my app will be accessible only into the cluster
- NodePort - gives ip to node and a port to pod and my app will be accessible from outside the cluster
- ExternalName - gives DNS name record and my app will be accessible from outside the cluster by using the DNS name
- LoadBalancer - adds LoadBalancer

# SERVICE CREATION
// to create a service i need to create deployment first
$kubectl create deployment myapp_depl --image=ag22222222/k8s:latest --replicas=3

# CREATE ClusterIP SERVICE
// there are no need to create a service for clusterip, k8s creates it automatically
$kubectl expose deployment myapp_depl --type=ClusterIP --port=8080

## to watch services
$kubectl get services
// or
$kubectl get svc

## to delete service
$kubectl delete service myapp_depl

# CREATE NodePort
$kubectl expose deployment myapp_depl --type=NodePort --port=8080
// it creates a one port for all nodes in deployment
// but every node will have a different ip

## to output the ip and port of each node
$kubectl describe nodes | grep ExternalIP

## to open an app on node just type in into browser http://<node_ip>:<node_port>

# CREATE LoadBalancer
$kubectl expose deployment myapp_depl --type=LoadBalancer --port=8080
