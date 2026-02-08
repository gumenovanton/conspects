# EXAMPLE
// if i create an app
// if i want to run it on a startup and restart it if it fails
// i need to create a service file

// for example i create an app, here i just create a bash script
$ vi /usr/local/bin/myapp.sh

// jush write two log messages to logs
// one when app starts with info priority
// sleep 10 seconds
// and one when app crashes with error priority
#!/bin/bash
echo "MyApp Started" | systemd-cat -t MyApp -p info
sleep 10
echo "MyApp Crashed" | systemd-cat -t MyApp -p error

// make it executable
chmod +x /usr/local/bin/myapp.sh

# SERVICE FILE CREATION
// service files lives in /lib/systemd/system/
// i can use one as a template
// when i create my own service file i must place it into /etc/systemd/system/

// To find all instructions related to services, use the following command:
$ man systemctl.service

[Unit]
Description=MyApp Service
Documentation=man:sshd // this is a link to the man page docs(just example)
After=network.target // runs only after network is up
ConditionPathExists=/usr/local/bin/myapp.sh //run my service only if this file exists


[Service]
EnvironmentFile=/etc/myapp.env - all env virables from this file will be loaded
ExecStartPre=echo "Pre-start message" // what to do before starting the service
ExecStart=/usr/local/bin/myapp.sh // command to start my app
ExecReload=/usr/local/bin/myapp.sh // what to do when runs systemctl reload command
KillMode=process //how to stop, when to runs systemctl stop, process - means only kill the main process not the child processes
Restart=always //how to restart
RestartSec=1 // restart after 1 second
RestartPreventExitStatus=255 // not to restart if exit status is 255
Type=simple //notify - means if service notify when restarted, simple mens not notify
RuntimeDirectory=/var/run/myapp // directory for temporary files
RuntimeDirectoryMode=0755 // permissions for the directory

[Install]
WantedBy=multi-user.target // load only on multi-user.target
Allias=myapp.service // allias of link on my app service creates a sumlink in /etc directory

# RUN THE SERVICE
// reload systemctl deamon at first
$ sudo systemctl daemon-reload
$ sudo systemctl start myapp.service

// to see the logs of my app service
$ sudo journalctl -u myapp.service
// or
$ sudo journalctl -f
