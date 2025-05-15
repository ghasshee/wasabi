#![no_std]
#![no_main]
#![feature(offset_of)]

use core::panic::PanicInfo;
use core::time::Duration;

use wasabi::executor::block_on; 
use wasabi::executor::Executor;
use wasabi::executor::Task;
use wasabi::executor::TimeoutFuture;
use wasabi::hpet::global_timestamp;
use wasabi::init::init_basic_runtime;
use wasabi::init::init_paging;
use wasabi::init::init_hpet;
use wasabi::init::init_allocator; 
use wasabi::init::init_display;
use wasabi::println;
use wasabi::error;
use wasabi::info;
use wasabi::warn;
use wasabi::print::hexdump; 
use wasabi::print::set_global_vram;
use wasabi::qemu::exit_qemu;
use wasabi::qemu::QemuExitCode;
use wasabi::uefi::init_vram;
use wasabi::uefi::locate_loaded_image_protocol;
use wasabi::uefi::EfiHandle;
use wasabi::uefi::EfiMemoryType;
use wasabi::uefi::EfiSystemTable;
use wasabi::x86::flush_tlb;
use wasabi::x86::hlt;
use wasabi::x86::init_exceptions;
use wasabi::x86::read_cr0;
use wasabi::x86::read_cr3;
use wasabi::x86::read_cr4; 
use wasabi::x86::trigger_debug_interrupt;
use wasabi::x86::PageAttr;





#[no_mangle]
fn efi_main(image_handle: EfiHandle, efi_system_table: &EfiSystemTable) {
    println!("Booting WasabiOS...");
    println!("image_handle: {:#018X}", image_handle);
    println!("efi_system_table: {:#p}", efi_system_table);
    let loaded_image_protocol = 
        locate_loaded_image_protocol(image_handle, efi_system_table)
            .expect("Failed to get LoadedImageProtocol");
    println!("image_base: {:#018X}", loaded_image_protocol.image_base);
    println!("image_size: {:#018X}", loaded_image_protocol.image_size);
    info!("info");
    warn!("warn");
    error!("error");
    hexdump(efi_system_table);


    let mut vram = init_vram(efi_system_table).expect("init_vram failed");
    init_display(&mut vram); 




    // let mut w = BitmapTextWriter::new(&mut vram);
    set_global_vram(vram); 

    let acpi = efi_system_table.acpi_table().expect("ACPI table not found");

    let memory_map = init_basic_runtime(image_handle, efi_system_table);

    info!("Hello, Non-UEFI world!"); 
    info!("CR0 : {:064b}", read_cr0());

    init_allocator(&memory_map); 


    let (_gdt, _idt) = init_exceptions();


    info!("Just before write to CR3");
    info!("CR0 : {:064b}", read_cr0());

    init_paging(&memory_map);

    info!("Now we are using our own page tables!");
    info!("CR0 : {:064b}", read_cr0());
    

    

    info!("Flushing TLB using CR3...") ;
    flush_tlb();
    info!("CR0 : {:064b}", read_cr0());
    



    init_hpet(acpi); 

    let t0 = global_timestamp();
    let task1 = Task::new(async move {
        for i in 100..=103 {
            info!("{i} hpet.main_conter = {:?}", global_timestamp() - t0);
            //yield_execution().await;
            TimeoutFuture::new(Duration::from_secs(1)).await;
        }
        Ok(())
    });
    let task2 = Task::new(async move {
        for i in 200..=203 {
            info!("{i} hpet.main_conter = {:?}", global_timestamp() - t0);
            //yield_execution().await;
            TimeoutFuture::new(Duration::from_secs(2)).await;
        }
        Ok(())
    });


    let mut executor = Executor::new();
    executor.enqueue(task1);
    executor.enqueue(task2);
    Executor::run(executor)



/*
    loop {
        hlt()
    }
    */
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    error!("PANIC: {info:?}");
    exit_qemu(QemuExitCode::Fail);
}

