# Intel backlight setter
A simple service to change the brightness on intel backlight laptops. Or if you have other backlight manufacturer - you can change the file paths.

The service creates a named pipe at `/run/brightness`.

You can write values to this named pipe. Any bad value is ignored.

This service should have minimal CPU usage as it is waiting for the named pipe almost all the time.
