#!/bin/bash -e
mkdir -p mnt/EFI/BOOT/
# cp generated/bin/os.efi mnt/EFI/BOOT/BOOTX64.EFI
TMPDIR=`mktemp -d`
DISK=/dev/sda1

read -p "Write WasabiOS to ${DISK}. Are you sure? [Enter to proceed, or Ctrl-C to abort] " REPLY
sudo mount ${DISK} ${TMPDIR}
sudo cp -rf mnt/* ${TMPDIR}/
sudo umount ${DISK}
