# Backlight setter
A simple service to change the brightness of the backlight on laptops (defaults to intel backlight, but can be changed in `main.rs`).

The service creates a named pipe at `/run/brightness` (can be changed in `main.rs`).

You can write number values to this named pipe or `inc` / `dec` for step increase / decrease. Any bad value is ignored.

This service should have minimal CPU usage as it is waiting for the named pipe almost all the time.

When the release build is moved to `/usr/bin` you can add the following service to `/usr/lib/systemd/system/intel-backlight-setter.service`:
```
[Unit]
Description=Intel backlight setter service

[Service]
ExecStart=intel-backlight-setter

[Install]
WantedBy=multi-user.target
```
Keep the privileges of the executable in mind when doing this.
