#!/bin/bash

for baud in 1200 2400 4800 9600 19200 38400; do
    echo "Using ${baud} baud."
    stty -F /dev/ttyUSB0 "${baud}" cs8 -cstopb -parenb
    echo -e "\x01\x06\x00\xFF\x00\x05\x79\xF9" > /dev/ttyUSB0
    sleep 1
done
