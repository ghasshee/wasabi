#!/bin/bash


dd if=/dev/zero of=disk.img bs=512 count=93750
echo "null image created"

parted disk.img -s -a minimal mklabel gpt 
echo "gpt label created" 

parted disk.img -s -a minimal mkpart EFI FAT16 2048s 93716s
echo "initial partition created" 

parted disk.img -s -a minimal toggle 1 boot

echo "boot on" 

dd if=/dev/zero of=./part.img bs=512 count=91669

echo "null part.img is created"

mformat -i ./part.img -h 32 -t 32 -n 64 -c 1

echo "format it with FAT16" 

mcopy -i ./part.img mnt/EFI/BOOT/BOOTX64.EFI ::

echo "copying wasabi.efi to it"

dd if=./part.img of=./disk.img bs=512 count=91669 seek=2048 conv=notrunc

echo "now disk.img is created!"

rm -f part.img
