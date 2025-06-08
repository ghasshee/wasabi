os: 
	cargo build --target x86_64-unknown-uefi 

link: os 
	mkdir -p mnt/EFI/BOOT
	cp target/x86_64-unknown-uefi/debug/wasabi.efi mnt/EFI/BOOT/BOOTX64.EFI

run: link
	qemu-system-x86_64 -bios third_party/ovmf/RELEASEX64_OVMF.fd \
		-drive format=raw,file=fat:rw:mnt

install: os 
	scripts/install.sh 

clean: 
	cargo clean
	rm Cargo.lock
	sudo rm disk.img

