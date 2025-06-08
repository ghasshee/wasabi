extern crate alloc;


use crate::error::Error;
use crate::error::Result;
use crate::acpi::AcpiRsdpStruct;

use crate::graphics::Bitmap;
// use crate::result::Result;
use crate::info;
// use alloc::string::String;
use core::mem::offset_of;
use core::mem::size_of;
use core::pin::Pin; 
use core::ptr::null_mut;

use core::marker::PhantomPinned; 




const EFI_GRAPHICS_OUTPUT_PROTOCOL_GUID: EfiGuid = EfiGuid {
    data0: 0x9042a9de,
    data1: 0x23dc,
    data2: 0x4a38,
    data3: [0x96, 0xfb, 0x7a, 0xde, 0xd0, 0x80, 0x51, 0x6a],
};

const EFI_LOADED_IMAGE_PROTOCOL_GUID: EfiGuid = EfiGuid {
    data0: 0x5B1B31A1,
    data1: 0x9562,
    data2: 0x11d2,
    data3: [0x8E, 0x3F, 0x00, 0xA0, 0xC9, 0x69, 0x72, 0x3B],
};

const EFI_ACPI_TABLE_GUID: EfiGuid = EfiGuid {
    data0: 0x8868e871,
    data1: 0xe4f1,
    data2: 0x11d3,
    data3: [0xbc, 0x22, 0x00, 0x80, 0xc7, 0x3c, 0x88, 0x81],
};

type EfiVoid = u8;

pub type EfiHandle = u64;


#[repr(C)]
#[derive(Debug)]
pub struct EfiTableHeader {
    pub signature: u64,
    pub revision: u32,
    pub header_size: u32,
    pub crc32: u32,
    reserved: u32,
}



#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[must_use]
#[repr(u64)]
pub enum EfiStatus {
    Success = 0,
}

impl EfiStatus {
    pub fn into_result(self) -> Result<()> {
        if self == EfiStatus::Success {
            Ok(())
        } else {
            Err(self.into())
        }
    }
}


pub union EfiBootServicesTableHandleProtocolVariants {
    pub handle_loaded_image_protocol: extern "win64" fn(
        handle: EfiHandle, 
        protocol: &EfiGuid,
        interface: *mut Option<Pin<&EfiLoadedImageProtocol>>,
    ) -> EfiStatus,
  /*  pub handle_simple_file_system_protocol: extern "win64" fn(
        handle: EfiHandle,
        protocol: *const EfiGuid,
        interface: *mut Option<Pin<&EfiSimpleFileSystemProtocol>>,
    ) -> EfiStatus, */
}




#[allow(dead_code)] 
#[repr(usize)]
pub enum AllocType {
    AnyPages = 0,
    MaxAddress,
    Address,
}

#[allow(dead_code)]
#[repr(u32)]
pub enum MemoryType {
    Reserved = 0,
    LoaderCode,
    LoaderData,
    BootServicesCode,
    BootServicesData,
    RuntimeServicesCode,
    RuntimeServicesData,
    ConventionalMemory,
    UnusableMemory,
    ACPIReclaimMemory,
    ACPIMemoryNVS,
    MemoryMappedIO,
    MemmoryMappedIOPortSpace,
    PalCode,
    PersistentMemory,
}




        

#[repr(C)]
pub struct EfiBootServicesTable {
    //_reserved0: [u64; 40],
    //_reserved0: [u64; 7],
    _header: EfiTableHeader, 
    _reserved0: [u64; 2],
    allocate_pages: extern "win64" fn(
        allocate_type: AllocType,
        memory_type: MemoryType,
        pages: usize, 
        mem: &mut *mut u8,) -> EfiStatus, 
    _reserved1: [u64; 1], 
    get_memory_map: extern "win64" fn(
        memory_map_size: *mut usize,
        memory_map: *mut u8,
        map_key: *mut usize,
        descriptor_size: *mut usize,
        descriptor_version: *mut u32,) -> EfiStatus,
    //_reserved1: [u64; 32],
    //_reserved1: [u64; 21],
    _reserved2: [u64;11],
    handle_protocol: EfiBootServicesTableHandleProtocolVariants,
    // handle_protocol: extern "win64" fn(
    //     handle: EfiHandle,
    //     protocol: *const EfiGuid,
    //     interface: *mut *mut EfiVoid,
    //     ) -> EfiStatus,
    _reserved3: [u64; 9],
    exit_boot_services: extern "win64" fn(
        image_handle: EfiHandle, 
        map_key: usize) -> EfiStatus,
    _reserved4: [u64; 10],
    locate_protocol: extern "win64" fn(
        protocol: *const EfiGuid,
        registration: *const EfiVoid,
        interface: *mut *mut EfiVoid,) -> EfiStatus,
    _pinned: PhantomPinned,
}
impl EfiBootServicesTable {

    pub fn get_memory_map(&self, map: &mut MemoryMapHolder) -> EfiStatus {
        (self.get_memory_map)(
            &mut map.memory_map_size,
            map.memory_map_buffer.as_mut_ptr(),
            &mut map.map_key,
            &mut map.descriptor_size,
            &mut map.descriptor_version,
        )
    }
    pub fn handle_loaded_image_protocol(
        &self,
        image_handle: EfiHandle,
    ) -> Result<Pin<&'static EfiLoadedImageProtocol>> {
        let mut loaded_image_protocol = None;
        let handle_protocol = &self.handle_protocol;
        let status = unsafe {
            (handle_protocol.handle_loaded_image_protocol)(
                image_handle,
                &EFI_LOADED_IMAGE_PROTOCOL_GUID,
                &mut loaded_image_protocol,
                )
        };
        status.into_result()?;
        loaded_image_protocol
            .ok_or(Error::Failed("hoge"))
    }
}

const _:() = assert!(offset_of!(EfiBootServicesTable, get_memory_map) == 56);
const _:() = assert!(offset_of!(EfiBootServicesTable, exit_boot_services) == 232); 
const _:() = assert!(offset_of!(EfiBootServicesTable, locate_protocol) == 320);


#[repr(C)]
#[derive(Copy, Clone)]
pub struct EfiConfigurationTable {
    vendor_guid: EfiGuid,
    pub vendor_table: *const u8,
}

pub struct EfiSimpleTextOutputProtocol {
    reset: EfiHandle,
    output_string: extern "win64" 
        fn(this: *const EfiSimpleTextOutputProtocol, str: *const u16) -> EfiStatus,
    test_string: EfiHandle,
    set_mode: EfiHandle,
    set_attribute: EfiHandle,
    clear_screen: extern "win64" 
        fn(thins: *const EfiSimpleTextOutputProtocol) -> EfiStatus,
    //
    _pinned: PhantomPinned
} 


#[repr(C)]
pub struct EfiSystemTable {
    //_reserved0: [u64; 12],
    header: EfiTableHeader,
    firmware_vendor: EfiHandle,
    firmware_revision: u32, 
    console_in_handle: EfiHandle,
    con_in: EfiHandle,
    console_out_Handle: EfiHandle,
    con_out: Pin<&'static EfiSimpleTextOutputProtocol>,
    standart_error_handle: EfiHandle,
    std_err: EfiHandle,
    runtime_services: EfiHandle, 
    boot_services: Pin<&'static EfiBootServicesTable>, 
    number_of_table_entries: usize,
    configuration_table: *const EfiConfigurationTable,
    _pinned: PhantomPinned, 
}

//const _: () = assert!(offset_of!(EfiSystemTable, boot_services) == 96);

impl EfiSystemTable {
    pub fn con_out(&self) -> Pin<&EfiSimpleTextOutputProtocol> {
        self.con_out
    }
    pub fn boot_services(&self) -> Pin<&EfiBootServicesTable> {
        self.boot_services
    }
    fn lookup_config_table(
        &self,
        guid: &EfiGuid,
    ) -> Option<EfiConfigurationTable> {
        for i in 0 .. self.number_of_table_entries {
            let ct = unsafe { &*self.configuration_table.add(i) };
            if ct.vendor_guid == *guid {
                return Some(*ct);
            }
        }
        None
    }
    pub fn acpi_table(&self) -> Option<&'static AcpiRsdpStruct> {
        self.lookup_config_table(&EFI_ACPI_TABLE_GUID)
            .map(|t| unsafe { &*(t.vendor_table as *const AcpiRsdpStruct) })
    }
}


#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct EfiGuid {
    pub data0: u32,
    pub data1: u16,
    pub data2: u16,
    pub data3: [u8; 8],
}





#[repr(C)]
#[derive(Debug)]
struct EfiGraphicsOutputProtocol<'a> {
    reserved: [u64; 3],
    pub mode: &'a EfiGraphicsOutputProtocolMode<'a>,
}

#[repr(C)]
#[derive(Debug)]
struct EfiGraphicsOutputProtocolMode<'a> {
    pub max_mode: u32,
    pub mode: u32,
    pub info: &'a EfiGraphicsOutputProtocolPixelInfo,
    pub size_of_info: u64,
    pub frame_buffer_base: usize,
    pub frame_buffer_size: usize,
}

#[repr(C)]
#[derive(Debug)]
struct EfiGraphicsOutputProtocolPixelInfo {
    version: u32,
    pub horizontal_resolution: u32,
    pub vertical_resolution: u32,
    _padding0: [u32; 5],
    pub pixels_per_scan_line: u32,
}
const _: () = assert!(size_of::<EfiGraphicsOutputProtocolPixelInfo>() == 36);

fn locate_graphic_protocol<'a>(efi_system_table: &EfiSystemTable,) 
    -> Result<&'a EfiGraphicsOutputProtocol<'a>> {
    let mut graphic_output_protocol = null_mut::<EfiGraphicsOutputProtocol>();
    let status = (efi_system_table.boot_services.locate_protocol)(
        &EFI_GRAPHICS_OUTPUT_PROTOCOL_GUID,
        null_mut::<EfiVoid>(),
        &mut graphic_output_protocol as *mut *mut EfiGraphicsOutputProtocol as *mut *mut EfiVoid,
    );
    if status != EfiStatus::Success {
        return Err("Failed to locate graphics output protocol".into());
    }
    Ok(unsafe { &*graphic_output_protocol })
}

#[repr(C)] 
#[derive(Debug)]
pub struct EfiLoadedImageProtocol {
    _reserved0: [u64; 3],
    pub device_handle: EfiHandle, 
    _reserved1: [u64; 4], 
    pub image_base: u64,
    pub image_size: u64,
    _pinned: PhantomPinned, 
}
const _: () = assert!(offset_of!(EfiLoadedImageProtocol, device_handle) == 24);

/* 
pub fn locate_loaded_image_protocol(
    image_handle: EfiHandle,
    efi_system_table: &EfiSystemTable,
    )-> Result<&EfiLoadedImageProtocol> {
    let mut loaded_image_protocol = null_mut::<EfiLoadedImageProtocol>();
    let status = (efi_system_table.boot_services.handle_protocol)(
        image_handle,
        &EFI_LOADED_IMAGE_PROTOCOL_GUID,
        &mut loaded_image_protocol as *mut *mut EfiLoadedImageProtocol
            as *mut *mut EfiVoid,
    );
    if status != EfiStatus::Success {
        return Err("Failed to locate graphics output protocol");
    }
    Ok(unsafe {&*loaded_image_protocol })
}
*/






#[repr(C)] 
#[derive(Clone, Copy, PartialEq, Eq, Debug)] 
pub struct EfiMemoryDescriptor {
    memory_type: EfiMemoryType,
    physical_start: u64,
    virtual_start: u64,
    number_of_pages: u64,
    attribute: u64,
}
impl EfiMemoryDescriptor {
    pub fn memory_type(&self) -> EfiMemoryType {
        self.memory_type
    }
    pub fn number_of_pages(&self) -> u64 {
        self.number_of_pages
    }
    pub fn physical_start(&self) -> u64 {
        self.physical_start
    }
}

const MEMORY_MAP_BUFFER_SIZE: usize = 0x8000;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum EfiMemoryType {
    RESERVED = 0,
    LOADER_CODE,
    LOADER_DATA,
    BOOT_SERVECES_CODE,
    BOOT_SERVECES_DATA,
    RUNTIME_SERVECES_CODE,
    RUNTIME_SERVECES_DATA,
    CONVENTIONAL_MEMORY,
    UNUSABLE_MEMORY,
    ACPI_RECLAIM_MEMORY,
    ACPI_MEMORY_NVS,
    MEMORY_MAPPED_IO,
    MEMORY_MAPPED_IO_PORT_SPACE,
    PAL_CODE,
    PERSISTENT_MEMORY,
}



pub struct MemoryMapHolder { 
    memory_map_buffer: [u8; MEMORY_MAP_BUFFER_SIZE],
    memory_map_size: usize,
    map_key: usize,
    descriptor_size: usize,
    descriptor_version: u32,
}
pub struct MemoryMapIterator<'a> {
    map: &'a MemoryMapHolder, 
    ofs: usize,
}

impl<'a> Iterator for MemoryMapIterator<'a> {
    type Item = &'a EfiMemoryDescriptor;
    fn next(&mut self) -> Option<&'a EfiMemoryDescriptor> {
        if self.ofs >= self.map.memory_map_size {
            None
        } else {
            let e: &EfiMemoryDescriptor = unsafe {
                &*(self.map.memory_map_buffer.as_ptr().add(self.ofs) 
                   as *const EfiMemoryDescriptor)
            };
            self.ofs += self.map.descriptor_size;
            Some(e) 
        }
    }
}

impl MemoryMapHolder {
    pub const fn new() -> MemoryMapHolder {
        MemoryMapHolder {
            memory_map_buffer: [0; MEMORY_MAP_BUFFER_SIZE],
            memory_map_size: MEMORY_MAP_BUFFER_SIZE,
            map_key: 0,
            descriptor_size: 0,
            descriptor_version: 0,
        }
    }
    pub fn iter(&self) -> MemoryMapIterator {
        MemoryMapIterator { map: self, ofs: 0 } 
    }

}




#[derive(Clone, Copy)]
pub struct VramBufferInfo {
    buf: *mut u8,
    width: i64,
    height: i64,
    pixels_per_line: i64,
}

impl Bitmap for VramBufferInfo {
    fn bytes_per_pixel(&self)   -> i64 { 4 }
    fn pixels_per_line(&self)   -> i64 { self.pixels_per_line } 
    fn width(&self)             -> i64 { self.width }
    fn height(&self)            -> i64 { self.height } 
    fn buf_mut(&mut self)       -> *mut u8 { self.buf }
} 

pub fn init_vram(efi_system_table: &EfiSystemTable) -> Result<VramBufferInfo> {
    let gp = locate_graphic_protocol(efi_system_table)?;
    Ok(VramBufferInfo {
        buf:    gp.mode.frame_buffer_base as *mut u8,
        width:  gp.mode.info.horizontal_resolution as i64,
        height: gp.mode.info.vertical_resolution as i64,
        pixels_per_line: gp.mode.info.pixels_per_scan_line as i64,
    }) 
}


    
pub fn exit_from_efi_boot_services(
    image_handle: EfiHandle,
    efi_system_table: Pin<&EfiSystemTable>,
    memory_map: &mut MemoryMapHolder,
    ) {
    loop {
        let status = efi_system_table
            .as_ref()
            .boot_services
            .get_memory_map(memory_map);
        assert_eq!(status, EfiStatus::Success);
        info!("get_memory_map" ); 
        let status = (efi_system_table.as_ref().boot_services.exit_boot_services)(
            image_handle,
            memory_map.map_key,
            );
        info!("exit_boot_services DONE");
        if status == EfiStatus::Success {
            info!("EfiStatus == Success; Just breaking the uefi exit loop"); 
            break;
        }
    }
}

