


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

