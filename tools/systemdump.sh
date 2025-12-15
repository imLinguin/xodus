#!/bin/bash
# Script for dumping raw smbios system struct and drive serial number 

if [ $(id -u) -ne 0 ]; then
    echo "Please run as root"
    exit
fi

# Dump DMI system info
dmi=$(cat /sys/firmware/dmi/entries/1-0/raw | base64 -w 0 -)

mount=$(findmnt -n -o SOURCE /)
serial=$(udevadm info --query=property "--name=$mount" | grep SERIAL_SHORT | cut -d= -f2-)

echo -n "{\"system\": \"$dmi\",\"drive_serial\": \"$serial\"}"