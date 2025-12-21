# REBOOT & SHUTDOWN
## to reboot the system
$ systemctl reboot

## to shutdown the system
$ systemctl poweroff

## to reboot and shutdown with force when some programs are frozen
// not recommended
$ systemctl reboot -f
$ systemctl poweroff -f

//if it is not working, use the following commands:
$ systemctl reboot --force --force
$ systemctl poweroff --force --force

# REBOOT & SHUTDOWN BY TIME
// shutdown at 02:00(24 hours)
$ sudo shutdown 02:00

// reboot at 03:00(24 hours)
$ sudo shutdown -r 03:00

// shutdown in  15 minutes
$ sudo shutdown +15

// reboot in  15 minutes
$ sudo shutdown -r +15

## shutdown with message
// i do not know where the message will be displayed
$ sudo shutdown +1 'System will shutdown in 15 minutes'

## reboot with message
// i do not know where the message will be displayed
$ sudo shutdown -r +1 'System will reboot in 15 minutes'
