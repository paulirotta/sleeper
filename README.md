# sleeper
Autonomous drift car experiment

1. Visit http://rustup.rs 

2. On Raspberry Pi, PWM hardware is usually for audio out. Some reconfig (and don't use audio at the same time..): 

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

5. (WIP Untested) From Linux you can cross-compile to Raspberry Pi if you:
    rustup target add armv7-unknown-linux-gnueabihf
    sudo apt-get install gcc-4.7-multilib-arm-linux-gnueabihf
    cargo build --target=armv7-unknown-linux-gnueabihf

6. If audio recording is needed, a USB mic can be setup up according to http://www.g7smy.co.uk/2013/08/recording-sound-on-the-raspberry-pi/ 
