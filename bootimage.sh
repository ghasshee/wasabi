#!/bin/bash

DISK=$1

sudo dd if=/dev/zero of=disk.img bs=512 count=93750
echo "null image created"

sudo parted disk.img -s -a minimal mklabel gpt 
echo "gpt label created" 

sudo parted disk.img -s -a minimal mkpart EFI FAT16 2048s 93716s
echo "initial partition created" 

sudo parted disk.img -s -a minimal toggle 1 boot

echo "boot on" 

sudo dd if=/dev/zero of=./part.img bs=512 count=91669

echo "null part.img is created"

sudo mformat -i ./part.img -h 32 -t 32 -n 64 -c 1

echo "format it with FAT16" 

sudo mcopy -i ./part.img mnt/EFI/BOOT/BOOTX64.EFI ::

echo "copying wasabi.efi to it"

sudo dd if=./part.img of=./disk.img bs=512 count=91669 seek=2048 conv=notrunc

echo "now disk.img is created!"

sudo rm -f part.img

echo "before writing to $DISK, zeroing the $DISK"

sudo dd if=/dev/zero of=$DISK count=100000 bs=512

echo "writing disk.img to $DISK" 
sudo dd if=./disk.img of=$DISK 

echo "mounting ${DISK}1 to /mnt" 

sudo mount "${DISK}1" /mnt

tree /mnt > /dev/stdout 

sudo mkdir -p /mnt/EFI/BOOT

echo "we made new dir /mnt/EFI/BOOT" 

tree /mnt > /dev/stdout

sudo mv /mnt/BOOTX64.EFI /mnt/EFI/BOOT/BOOTX64.EFI

echo "now we have arranged wasabi.efi as the /EFI/BOOT/BOOTX64.EXE"

tree /mnt > /dev/stdout


sudo umount /mnt
echo "we umounted /mnt" 

tree /mnt > /dev/stdout

[ -e /mnt/EFI ] && sudo rm -rf /mnt/EFI


echo "removing disk image .." 
sudo rm disk.img
