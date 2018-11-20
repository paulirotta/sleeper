# sleeper
Autonomous drift car experiment

1. Visit http://rustup.rs 

2. On Raspberry Pi: 

Instructions at https://docs.golemparts.com/rppal/0.9.0/rppal/spi/index.html which are, briefly:
A. Add dtoverlay=pwm-2chan to /boot/config.txt
B. Append the following snippet to /etc/udev/rules.d/99-com.rules. Reboot the Raspberry Pi afterwards.

SUBSYSTEM=="pwm*", PROGRAM="/bin/sh -c '\
    chown -R root:gpio /sys/class/pwm && chmod -R 770 /sys/class/pwm;\
    chown -R root:gpio /sys/devices/platform/soc/*.pwm/pwm/pwmchip* &&\
    chmod -R 770 /sys/devices/platform/soc/*.pwm/pwm/pwmchip*\
'"

3. sudo apt install libatlas-base-dev

4. pip3 install tensorflow

WIP 5. You can run on a PC without PWM hardware since the "gpio" feature is off by default. On a Pi, use
    cargo run --feature "gpio" --release

