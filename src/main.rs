#![no_std]
#![no_main]
#![feature(offset_of)]

use core::arch::asm;
// use core::cmp::min;
// use core::fmt;
use core::fmt::Write;
// use core::mem::offset_of;
// use core::mem::size_of;
use core::panic::PanicInfo;
// use core::ptr::null_mut;
use core::writeln;
//use core::slice;

use wasabi::graphics::Bitmap;
use wasabi::graphics::fill_rect;
use wasabi::graphics::draw_test_pattern;
//use wasabi::result::Result;

use wasabi::init::init_basic_runtime;

use wasabi::println;
use wasabi::error;
use wasabi::info;
use wasabi::warn;

use wasabi::print::hexdump; 

use wasabi::qemu::exit_qemu;
use wasabi::qemu::QemuExitCode;

//use wasabi::serial::SerialPort;

// use wasabi::uefi::exit_from_efi_boot_services;
use wasabi::uefi::init_vram;
use wasabi::uefi::EfiHandle;
use wasabi::uefi::EfiMemoryType;
use wasabi::uefi::EfiSystemTable;
// use wasabi::uefi::MemoryMapHolder;
use wasabi::uefi::VramTextWriter;

use wasabi::x86::hlt;

//type Result<T> = core::result::Result<T, &'static str>;

/*
pub fn hlt() {
    unsafe { asm!("hlt") }
}
*/

#[no_mangle]
//fn efi_main(_image_handle: EfiHandle, efi_system_table: &EfiSystemTable) {
fn efi_main(image_handle: EfiHandle, efi_system_table: &EfiSystemTable) {
    println!("Booting WasabiOS...");
    println!("image_handle: {:#018X}", image_handle);
    println!("efi_system_table: {:#p}", efi_system_table);
    info!("info");
    warn!("warn");
    error!("error");
    hexdump(efi_system_table);

 //   let mut sw = SerialPort :: new_for_com1();
 //   writeln!(sw, "Hello via serial port").unwrap();
    /*
    let efi_graphics_output_protocol = locate_graphic_protocol(efi_system_table).unwrap();
    let vram_addr = efi_graphics_output_protocol.mode.frame_buffer_base;
    let vram_byte_size = efi_graphics_output_protocol.mode.frame_buffer_size;
    let vram = unsafe {
        slice::from_raw_parts_mut(vram_addr as *mut u32, vram_byte_size / size_of::<u32>())
    };
    for e in vram {
        *e = 0xffffff;
    }
    */

    let mut vram = init_vram(efi_system_table).expect("init_vram failed");
    /* 
    for y in 0..vram.height {
        for x in 0..vram.width {
            if let Some(pixel) = vram.pixel_at_mut(x,y) {
                *pixel = 0x00ff00;
            }
        }
    }


    for y in 0 .. vram.height / 2 {
        for x in 0 .. vram.width / 2 {
            if let Some(pixel) = vram.pixel_at_mut(x,y) {
                *pixel = 0xff0000;
            }
        }
    }
    */

    let vw = vram.width();
    let vh = vram.height();
    fill_rect(&mut vram, 0x000000,  0,  0, vw, vh).expect("fill_rect failed");
    /*
    fill_rect(&mut vram, 0xff0000, 32, 32, 32, 32).expect("fill_rect failed");
    fill_rect(&mut vram, 0x00ff00, 64, 64, 64, 64).expect("fill_rect failed");
    fill_rect(&mut vram, 0x0000ff,128,128,128,128).expect("fill_rect failed");
    for i in 0 .. 256 {
        let _ = draw_point(&mut vram, 0x010101 * i as u32, i, i);
    }

    let grid_size: i64 = 32;
    let rect_size: i64 = grid_size * 8;
    for i in (0..=rect_size).step_by(grid_size as usize) {
        let _ = draw_line(&mut vram, 0xff0000, 0, i, rect_size, i);
        let _ = draw_line(&mut vram, 0xff0000, i, 0, i, rect_size);
    }
    let cx = rect_size / 2;
    let cy = rect_size / 2;
    for i in (0..=rect_size).step_by(grid_size as usize) {
        let _ = draw_line(&mut vram, 0xffff00, cx, cy, 0, i);
        let _ = draw_line(&mut vram, 0x00ffff, cx, cy, i, 0);
        let _ = draw_line(&mut vram, 0xff00ff, cx, cy, rect_size, i);
        let _ = draw_line(&mut vram, 0xffffff, cx, cy, i, rect_size);
    }
    */


/* 
    let font_a = "
........
...**...
...**...
...**...
...**...
..*..*..
..*..*..
..*..*..
..*..*..
.******.
.*....*.
.*....*.
.*....*.
***..***
........
........
";
    for (y,row) in font_a.trim().split('\n').enumerate() {
        for (x,pixel) in row.chars().enumerate() {
            let color = match pixel {
                '*' => 0xffffff,
                _   => continue,
            };
            let _ = draw_point(&mut vram, color, x as i64, y as i64);
        }
    }
*/

/*
    for (i,c) in "ABCDEF".chars().enumerate() {
        draw_font_fg(&mut vram, i as i64 * 16 + 256, i as i64 * 16, 0xffffff, c)
    }
*/

//    draw_str_fg(&mut vram, 256, 256, 0xffffff, "Hello, world!");

    draw_test_pattern(&mut vram);

    let mut w = VramTextWriter::new(&mut vram);
//    for i in 0..4 {
//        writeln!(w, "i = {i}").unwrap();
//    }

    /* 
    let mut memory_map = MemoryMapHolder::new();
    let status = efi_system_table
        .boot_services
        .get_memory_map(&mut memory_map);
    writeln!(w, "{status:?}").unwrap();
    */
    let memory_map = init_basic_runtime(image_handle, efi_system_table);
    let mut total_memory_pages = 0;
    for e in memory_map.iter() {
        if e.memory_type() != EfiMemoryType::CONVENTIONAL_MEMORY {
            continue;
        }
        total_memory_pages += e.number_of_pages(); 
        writeln!(w, "{e:?}").unwrap();
    }
    let total_memory_size_MiB = total_memory_pages * 4096 / 1024 / 1024;
    writeln!(w,
             "Total: {total_memory_pages} pages = {total_memory_size_MiB} MiB"
             ).unwrap();


    // println!("Hello, world!");
    /*
    exit_from_efi_boot_services(
        image_handle,
        efi_system_table,
        &mut memory_map,
        );
        */
    writeln!(w, "Hello, Non-UEFI world!").unwrap();

    loop {
        hlt()
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    error!("PANIC: {info:?}");
    exit_qemu(QemuExitCode::Fail);
}

