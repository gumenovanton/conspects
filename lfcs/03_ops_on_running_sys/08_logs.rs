# WHERE THE PROCESS LOGS
$ grep -r 'ssh' /var/log/
// or find where the app stores its logs in docs


# JOURNALCTL
// all logs
$ journalctl

// end of log file (10 logs)
$ journalctl -e

// falow mode, watch logs live
$ journalctl -f

// logs of a bynary
// find where the binary lives
$ which sudo
/bin/sudo

// show its logs
$ journalctl /bin/sudo

// logs of a unit
$ journalctl -u cups.service

// search logs by a log level
$ journalctl -p err

// get list of the log levels
$ journalctl -p
// alert crit debug emerg err info notice warning

// get info logs that starts with b literal
$ journalctl -p info -g '^b'

// logs after time
$ journalctl -S 02:00

// logs between time
$ journalctl -S 02:00 -U 03:00

// with dates
$ journalctl -S '2021-11-16 02:00:44'

// logs of the current boot
$ journalctl -b 0

// logs of the previous boot
$ journalctl -b -1

# LAST
// who loged in into the system
$ last

# LASTLOG
// who has last logs
$ lastlog
