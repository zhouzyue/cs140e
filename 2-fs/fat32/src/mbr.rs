use Error::{BadSignature, Io};
use std::{fmt, io};
use traits::BlockDevice;

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct CHS {
    // FIXME: Fill me in.
    head: u8,
    sector_and_high_bits_cylinder: u8,
    cylinder: u8
}

impl fmt::Debug for CHS {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("CHS").finish()
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone)]
pub struct PartitionEntry {
    // FIXME: Fill me in.
    boot_indicator: u8,
    starting_chs: CHS,
    ending_chs: CHS,
    partition_type: u8,
    pub relative_sector: u32,
    sectors: u32,
}

/// The master boot record (MBR).
#[repr(C, packed)]
pub struct MasterBootRecord {
    __bootstrap: [u8; 436],
    disk_id: [u8; 10],
    partitions: [PartitionEntry; 4],
    signature: u16
}

#[derive(Debug)]
pub enum Error {
    /// There was an I/O error while reading the MBR.
    Io(io::Error),
    /// Partition `.0` (0-indexed) contains an invalid or unknown boot indicator.
    UnknownBootIndicator(u8),
    /// The MBR magic signature was invalid.
    BadSignature,
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::Io(error)
    }
}

impl MasterBootRecord {
    /// Reads and returns the master boot record (MBR) from `device`.
    ///
    /// # Errors
    ///
    /// Returns `BadSignature` if the MBR contains an invalid magic signature.
    /// Returns `UnknownBootIndicator(n)` if partition `n` contains an invalid
    /// boot indicator. Returns `Io(err)` if the I/O error `err` occured while
    /// reading the MBR.
    pub fn from<T: BlockDevice>(mut device: T) -> Result<MasterBootRecord, Error> {
        let mut sector = [0u8; 512];
        let bytes = device.read_sector(0, &mut sector)?;

        if bytes != 512 {
            return Err(Error::Io(io::Error::new(io::ErrorKind::UnexpectedEof, "MBR should be 512 bytes")));
        }

        let mbr: MasterBootRecord = unsafe {
            std::mem::transmute(sector)
        };

        if mbr.signature != 0xAA55 {
            return Err(Error::BadSignature);
        }

        for i in 0..4 {
            let indicator = mbr.partitions[i].boot_indicator;
            if indicator != 0x80 && indicator != 0 {
                return Err(Error::UnknownBootIndicator(i as u8));
            }
        }

        Ok(mbr)
    }

    pub fn get_partition(&self, idx: usize) -> &PartitionEntry {
        &self.partitions[idx]
    }
}

impl fmt::Debug for MasterBootRecord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("MasterBootRecord")
            .field("disk id", &self.disk_id)
            .field("partitions", &self.partitions)
            .field("signature", &self.signature)
            .finish()
    }
}
