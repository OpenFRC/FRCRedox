#!/bin/bash

echo "Building tests"
cargo test --no-run --target arm-unknown-linux-gnueabi

echo "Downloading RoboRIO image"
wget "https://dl.dropboxusercontent.com/u/161436639/roborio.img"
./get_linux.sh

echo "Starting VM"
qemu-system-arm \
  -machine xilinx-zynq-a9 -cpu cortex-a9 -m 2G \
  -kernel linux/uImage -dtb linux/devicetree.dtb \
  -display none -serial null -serial mon:stdio \
  -localtime \
  -append "console=ttyPS0,115200 earlyprintk root=/dev/mmcblk0 rw" \
  -net user,hostfwd=tcp::10022-:22 \
  -net nic \
  -sd "roborio.img" > /dev/null &

sleep 60

echo "Running tests"
scp -P 10022 target/arm-unknown-linux-gnueabi/debug/frcredox-* lvuser@localhost:/home/lvuser/
ssh -p 10022 lvuser@localhost './frcredox-*'
