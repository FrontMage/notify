# Notify

`send notifications through http to linux machine`

Relies on `notify-send` command.

By default, it listens to 9527 port, Change it on `Rocket.toml`.

The icons directory is set to `$HOME/Documents/notify/icons`, change it on env variable `NOTIFY_ICONS`.

GET `/notify?<title>&<body>&<icon>&<expire>`

- title String
- body String
- icon String
- expire String
