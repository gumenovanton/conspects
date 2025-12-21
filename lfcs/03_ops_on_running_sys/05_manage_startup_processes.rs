# MANAGE STARTUP PROCESSES
// linux has init system, which is responsible for starting up the system and managing the startup processes.
// It calls the init system.
// In what order processes must be started.
// And what to do if a process fails to start.

// It has specific instructions to know what to do.
// They are called units.
// They are just text files that contain instructions on how to start a process.

## types of units
// service
// timer
// socket
// device
// target

# SERVICE
// tells the init system how it should manage an application.
// It can be started, stopped, restarted, or reloaded.

// To find all instructions related to services, use the following command:
$ man systemctl.service

## example
// if i run
$ systtemctl cat sshd.service
// it outputs
[Unit]
Description=OpenSSH Daemon
Wants=sshdgenkeys.service
After=sshdgenkeys.service
After=network.target

[Service]
Type=notify-reload
ExecStart=/usr/bin/sshd -D // what to do on start
KillMode=process
Restart=always // when to restart the service

[Install]
WantedBy=multi-user.target

## to edit the service unit file
$ sudo systemctl edit --full sshd.service

## to revert changes
$ sudo systemctl revert sshd.service

## to see all services to understand how it's named
$ sudo systemctl list-units --type service --all

## to see the status of the service
// we can see its status, active or not, pid, memory usage, command is run to start the service, and some logs.
$ sudo systemctl status sshd.service

## to stop the service
$ sudo systemctl stop sshd.service

## to start the service
$ sudo systemctl start sshd.service

## to restart the service
// when i change the service unit file i need to restart the service
$ sudo systemctl restart sshd.service

## to reload the service
// i can't restart when the service in use. In that case, i need to reload the service.
$ sudo systemctl reload sshd.service

## to reload at first and then restart the service
$ sudo systemctl reload-or-restart sshd.service

## to enable the service to start on boot
$ sudo systemctl enable sshd.service

## to disable the service from starting on boot
$ sudo systemctl disable sshd.service

## when i want to enable and start the service immediately
$ sudo systemctl enable --now sshd.service

## to disable and stop the service immediately
$ sudo systemctl disable --now sshd.service

## if a service does not want to stapping
// it because service one starts service two even if you disable it
// if you want to prevent it just mask it
$ sudo systemctl mask sshd.service

## to unmask it
$ sudo systemctl unmask sshd.service
