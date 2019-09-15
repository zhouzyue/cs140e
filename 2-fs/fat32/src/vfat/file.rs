use std::cmp::{max, min};
use std::io::{self, SeekFrom};

use traits;
use vfat::{Cluster, FatEntry, Metadata, Shared, Status, VFat};

#[derive(Debug)]
pub struct File {
    pub name: String,
    pub metadata: Metadata,
    start_cluster: Cluster,
    fs: Shared<VFat>,
    size: u32,
    offset: u32,
    curr_cluster: Option<Cluster>,
}

impl File {
    pub fn new(name: String, metadata: Metadata, start_cluster: Cluster, fs: Shared<VFat>, size: u32) -> File {
        File {
            name,
            metadata,
            start_cluster,
            fs,
            size,
            offset: 0,
            curr_cluster: Some(start_cluster),
        }
    }
}

// FIXME: Implement `traits::File` (and its supertraits) for `File`.
impl traits::File for File {
    fn sync(&mut self) -> io::Result<()> {
        unimplemented!("read-only system")
    }

    fn size(&self) -> u64 {
        self.size as u64
    }
}

impl io::Read for File {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let can_be_read = min(buf.len(), self.size as usize - self.offset as usize);
        let mut bytes_read = 0;
        let mut fs = self.fs.borrow_mut();
        let bytes_per_cluster = fs.bytes_per_sector as u32 * fs.sectors_per_cluster as u32;
        while bytes_read < can_be_read {
            let bytes = fs.read_cluster(
                self.curr_cluster.unwrap(),
                self.offset as usize % bytes_per_cluster as usize,
                &mut buf[bytes_read..can_be_read],
            )?;
            bytes_read += bytes;
            self.offset = self.offset + bytes as u32;

            if self.offset % bytes_per_cluster == 0 {
                let entry: &FatEntry = fs.fat_entry(self.curr_cluster.unwrap())?;
                let next_cluster = entry.next_cluster();
                self.curr_cluster = next_cluster;
            }
        }

        Ok(bytes_read)
    }
}

impl io::Write for File {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        unimplemented!()
    }

    fn flush(&mut self) -> io::Result<()> {
        unimplemented!()
    }
}

impl io::Seek for File {
    /// Seek to offset `pos` in the file.
    ///
    /// A seek to the end of the file is allowed. A seek _beyond_ the end of the
    /// file returns an `InvalidInput` error.
    ///
    /// If the seek operation completes successfully, this method returns the
    /// new position from the start of the stream. That position can be used
    /// later with SeekFrom::Start.
    ///
    /// # Errors
    ///
    /// Seeking before the start of a file or beyond the end of the file results
    /// in an `InvalidInput` error.
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        let seek_offset = match pos {
            SeekFrom::Current(offset) => self.offset + offset as u32,
            SeekFrom::End(offset) => self.size + offset as u32,
            SeekFrom::Start(offset) => offset as u32,
        };

        if seek_offset >= self.size {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, ""));
        } else {
            self.offset = seek_offset;
            let mut fs = self.fs.borrow_mut();
            let bytes_per_cluster = fs.bytes_per_sector as u32 * fs.sectors_per_cluster as u32;
            let cluster = self.offset / bytes_per_cluster;
            self.curr_cluster = Some(self.start_cluster);
            for i in 0..cluster {
                self.curr_cluster = fs.fat_entry(self.curr_cluster.unwrap())?.next_cluster();
            }
            Ok(self.offset as u64)
        }

    }
}
