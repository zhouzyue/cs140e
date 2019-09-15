use core::char::decode_utf16;
use std::ffi::OsStr;
use std::io;

use traits;
use util::VecExt;
use vfat::{Cluster, Entry, File, Shared, VFat};
use vfat::Metadata;

#[derive(Debug)]
pub struct Dir {
    cluster: Cluster,
    fs: Shared<VFat>,
    pub name: String,
    pub metadata: Metadata
}

#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
pub struct VFatRegularDirEntry {
    file_name: [u8; 8],
    file_ext: [u8; 3],
    pub metadata: Metadata,
    size: u32,
}

#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
pub struct VFatLfnDirEntry {
    sequence: u8,
    name: [u16; 5],
    attributes: u8,
    entry_type: u8,
    checksum: u8,
    name_2: [u16; 6],
    __r: u16,
    name_3: [u16; 2],
}

#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
pub struct VFatUnknownDirEntry {
    entry_type: u8,
    __r1: [u8; 10],
    attributes: u8,
    __r2: [u8; 20]
}

impl VFatUnknownDirEntry {
    fn is_deleted_or_unused(&self) -> bool {
        self.entry_type == 0xE5
    }

    fn prev_is_last_entry(&self) -> bool {
        self.entry_type == 0x00
    }

    fn is_regular_directory(&self) -> bool {
        (self.attributes & 0x10) == 0x10
    }

    fn is_lnf(&self) -> bool {
        self.attributes == (0x01 | 0x02 | 0x04 | 0x08)
    }
}


pub union VFatDirEntry {
    unknown: VFatUnknownDirEntry,
    regular: VFatRegularDirEntry,
    long_filename: VFatLfnDirEntry,
}

impl Dir {
    /// Finds the entry named `name` in `self` and returns it. Comparison is
    /// case-insensitive.
    ///
    /// # Errors
    ///
    /// If no entry with name `name` exists in `self`, an error of `NotFound` is
    /// returned.
    ///
    /// If `name` contains invalid UTF-8 characters, an error of `InvalidInput`
    /// is returned.
    pub fn find<P: AsRef<OsStr>>(&self, name: P) -> io::Result<Entry> {
        use traits::{Dir, Entry};

        let name_str = name.as_ref().to_str().unwrap();

        for t in self.entries().unwrap() {
            if name_str.eq_ignore_ascii_case(t.name()) {
                return Ok(t);
            }
        }
        Err(io::Error::new(io::ErrorKind::NotFound, "not found"))
    }

    pub fn new_root(fs: &Shared<VFat>) -> Dir {
        let cluster = fs.borrow().root_dir_cluster;
        Dir {
            cluster,
            fs: fs.clone(),
            name: String::new(),
            metadata: Metadata::default(),
        }
    }
}

impl traits::Dir for Dir {
    type Entry = Entry;
    type Iter = EntryIterator;

    fn entries(&self) -> io::Result<Self::Iter> {
        let mut buf = Vec::new();
        self.fs.borrow_mut().read_chain(self.cluster, &mut buf)?;
        Ok(EntryIterator {
            fs: self.fs.clone(),
            curr_idx: 0,
            data: unsafe { buf.cast() },
        })
    }
}

pub struct EntryIterator {
    fs: Shared<VFat>,
    curr_idx: usize,
    data: Vec<VFatDirEntry>,
}

impl Iterator for EntryIterator {
    type Item = Entry;

    fn next(&mut self) -> Option<Self::Item> {
        let mut raw_long_file_name = [0u16; 260];
        for _i in self.curr_idx..self.data.len() {
            let entry = self.data.get(self.curr_idx).unwrap();

            let unknown = unsafe { entry.unknown };

            if unknown.is_deleted_or_unused() {
                self.curr_idx += 1;
                continue;
            } else if unknown.prev_is_last_entry() {
                return None;
            }

            self.curr_idx += 1;
            if unknown.is_lnf() {
                let lnf = unsafe { entry.long_filename };

                let lnf_idx = (lnf.sequence & 0b11111) as usize - 1;
                let slot = lnf_idx * 13;

                raw_long_file_name[slot..slot + 5].copy_from_slice(&lnf.name);
                raw_long_file_name[slot + 5..slot + 11].copy_from_slice(&lnf.name_2);
                raw_long_file_name[slot + 11..slot + 13].copy_from_slice(&lnf.name_3);
            } else {
                let dir = unsafe { entry.regular };

                let file_name = if raw_long_file_name[0] == 0x00 {
                    let name = core::str::from_utf8(&dir.file_name).unwrap().trim_end();
                    let extension = core::str::from_utf8(&dir.file_ext).unwrap().trim_end();
                    let mut short_file_name = String::from(name);
                    if !extension.is_empty() {
                        short_file_name.push('.');
                        short_file_name.push_str(extension);
                    }
                    short_file_name
                } else {
                    let mut len = raw_long_file_name.len();
                    for i in 0..len {
                        if raw_long_file_name[i] == 0x00 || raw_long_file_name[i] == 0xFF {
                            len = i;
                            break;
                        }
                    }

                    let long_file_name = decode_utf16(raw_long_file_name[0..len].iter().cloned())
                        .map(|r| r.unwrap())
                        .collect::<String>();
                    long_file_name
                };

                if unknown.is_regular_directory() {
                    return Some(Entry::Dir(Dir {
                        cluster: Cluster::from(dir.metadata.start_cluster()),
                        fs: self.fs.clone(),
                        metadata: dir.metadata,
                        name: file_name,
                    }));
                } else {
                    return Some(Entry::File(File::new(
                        file_name,
                        dir.metadata,
                        Cluster::from(dir.metadata.start_cluster()),
                        self.fs.clone(),
                        dir.size,
                    )));
                }
            }
        }
        None
    }
}