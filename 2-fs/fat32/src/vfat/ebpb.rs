use std::{fmt, io};
use traits::BlockDevice;
use vfat::Error;

#[repr(C, packed)]
pub struct BiosParameterBlock {
    jmp_instruction: [u8; 3],
    oem_identifier: [u8; 8],
    pub bytes_per_sector: u16,
    pub sectors_per_cluster: u8,
    pub reserved_sectors: u16,
    pub number_of_fat: u8,
    // often 2
    max_directory_entries: u16,
    total_logical_sectors: u16,
    descriptor_type: u8,
    sectors_per_fat: u16,
    sectors_per_track: u16,
    heads_or_sides: u16,
    hidden_sectors: u32,
    total_logical_sectors_2: u32,

    pub sector_per_fat_32: u32,
    flags: u16,
    version: u16,
    pub root_dir_cluster_number: u32,
    fs_info_sector_number: u16,
    backup_boot_sector_number: u16,
    formatted_flag: [u8; 12],
    drive_number: u8,
    _reserved_flag: u8,
    signature: u8,
    _volume_id: u32,
    volume_label: [u8; 11],
    identifier_string: [u8; 8],
    boot_code: [u8; 420],
    bootable_partition_signature: u16
}

impl BiosParameterBlock {
    /// Reads the FAT32 extended BIOS parameter block from sector `sector` of
    /// device `device`.
    ///
    /// # Errors
    ///
    /// If the EBPB signature is invalid, returns an error of `BadSignature`.
    pub fn from<T: BlockDevice>(
        mut device: T,
        sector: u64
    ) -> Result<BiosParameterBlock, Error> {
        let mut buf = [0u8; 512];
        let bytes = device.read_sector(sector, &mut buf)?;

        if bytes != 512 {
            return Err(Error::Io(io::Error::new(io::ErrorKind::UnexpectedEof, "ebpb should be 512 bytes")));
        }

        let block: BiosParameterBlock = unsafe {
            std::mem::transmute(buf)
        };

        if block.bootable_partition_signature != 0xaa55 {
            return Err(Error::BadSignature);
        }

        Ok(block)
    }
}

impl fmt::Debug for BiosParameterBlock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("BiosParameterBlock")
            .finish()
    }
}
