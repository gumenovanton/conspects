# SWITCH BETWEEN BOOT TARGETS
// linux system has a default boot target, which can be changed using the systemctl command.
- graphical.target, // target with graphical interface
- multi-user.target, // target for multi-users without graphical interface
- rescue.target, // target with root shellfor(root password required)
- emergency.target, // target with read only filesystem(root password required)

## output default mode
$ systemctl get-default

## change to multi-user
// multi-user mode is a mode that allows multiple users to log in
// but without graphical interface.
$ sudo systemctl set-default multi-user.target
$ sudo systemctl reboot

## swiching immediately to a target mode for one session
$ sudo systemctl isolate graphical.target
