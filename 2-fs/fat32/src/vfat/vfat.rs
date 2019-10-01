use std::cmp::min;
use std::io;
use std::io::Write;
use std::mem::size_of;
use std::path::Component;
use std::path::Path;

use mbr::MasterBootRecord;
use traits::{BlockDevice, FileSystem};
use util::SliceExt;
use vfat::{Cluster, Dir, Entry, Error, FatEntry, File, Shared, Status};
use vfat::{BiosParameterBlock, CachedDevice, Partition};

#[derive(Debug)]
pub struct VFat {
    device: CachedDevice,
    pub bytes_per_sector: u16,
    pub sectors_per_cluster: u8,
    sectors_per_fat: u32,
    fat_start_sector: u64,
    data_start_sector: u64,
    pub root_dir_cluster: Cluster,
}

impl VFat {
    pub fn from<T>(mut device: T) -> Result<Shared<VFat>, Error>
        where T: BlockDevice + 'static
    {
        let mbr = MasterBootRecord::from(&mut device).unwrap();
        for i in 0..4 {
            let partition = mbr.get_partition(0);
            match partition.partition_type {
                0xB | 0xC => {
                    let partition_start = partition.relative_sector as u64;
                    let ebpb = BiosParameterBlock::from(&mut device, partition_start).unwrap();
                    let partition = Partition {
                        start: partition_start,
                        sector_size: ebpb.bytes_per_sector as u64,
                    };

                    let cached_device = CachedDevice::new(device, partition);
                    let fat_start_sector = partition_start + ebpb.reserved_sectors as u64;
                    let data_start_sector = fat_start_sector + ebpb.sector_per_fat_32 as u64 * ebpb.number_of_fat as u64;
                    let vfat = VFat {
                        device: cached_device,
                        bytes_per_sector: ebpb.bytes_per_sector,
                        sectors_per_cluster: ebpb.sectors_per_cluster,
                        sectors_per_fat: ebpb.sector_per_fat_32,
                        fat_start_sector,
                        data_start_sector,
                        root_dir_cluster: Cluster::from(ebpb.root_dir_cluster_number),
                    };
                    return Ok(Shared::new(vfat));
                }
                _ => {}
            }
        }
        Err(Error::Io(io::Error::new(io::ErrorKind::InvalidData, "fat32 partition not found")))
    }

    // TODO: The following methods may be useful here:
    //
    //  * A method to read from an offset of a cluster into a buffer.
    //
    pub fn read_cluster(
        &mut self,
        cluster: Cluster,
        offset: usize,
        mut buf: &mut [u8],
    ) -> io::Result<usize> {
        let sector = self.data_start_sector + cluster.index() as u64 * self.sectors_per_cluster as u64;
        let mut cur_sector = sector + offset as u64 / self.bytes_per_sector as u64;
        let mut bytes_read = 0;
        let bytes_can_be_read = min(buf.len(), self.sectors_per_cluster as usize * self.bytes_per_sector as usize - offset);
        let mut cur_offset = offset % self.bytes_per_sector as usize;
        while bytes_read < bytes_can_be_read {
            let data = self.device.get(cur_sector)?;
            bytes_read += buf.write(&data[cur_offset..])?;
            cur_sector += 1;
            cur_offset = 0;
        }

        Ok(bytes_can_be_read)
    }
    //
    //  * A method to read all of the clusters chained from a starting cluster
    //    into a vector.
    //
    pub fn read_chain(
        &mut self,
        start: Cluster,
        buf: &mut Vec<u8>,
    ) -> io::Result<usize> {
        let mut bytes_read = 0;

        let mut cur_cluster = start;
        let mut cluster_num = 0;

        loop {
            cluster_num += 1;
            let cur_entry = self.fat_entry(cur_cluster)?;
            match cur_entry.status() {
                Status::Data(next_cluster) => {
                    bytes_read += self.append_result(cur_cluster, buf, cluster_num)?;
                    cur_cluster = next_cluster;
                }
                Status::Eoc(_) => {
                    bytes_read += self.append_result(cur_cluster, buf, cluster_num)?;
                    return Ok(bytes_read);
                }
                _ => return Err(io::Error::new(io::ErrorKind::InvalidData, "invalid cluster chain")),
            }
        }
    }

    fn append_result(&mut self, cluster: Cluster, buf: &mut Vec<u8>, cluster_num: usize) -> io::Result<usize> {
        let bytes_per_cluster = self.bytes_per_sector as usize * self.sectors_per_cluster as usize;
        buf.resize(bytes_per_cluster * cluster_num, 0);
        self.read_cluster(cluster, 0, &mut buf[bytes_per_cluster * (cluster_num - 1)..])
    }

    //
    //  * A method to return a reference to a `FatEntry` for a cluster where the
    //    reference points directly into a cached sector.
    //
    pub fn fat_entry(&mut self, cluster: Cluster) -> io::Result<&FatEntry> {
        if !cluster.is_valid() {
            return Err(io::Error::new(io::ErrorKind::InvalidData, ""));
        }
        let fat_width = size_of::<FatEntry>();

        let cluster_in_fat_sector = cluster.id() * fat_width as u32 / self.bytes_per_sector as u32;
        let data = self.device.get(self.fat_start_sector + cluster_in_fat_sector as u64)?;

        let index = (cluster.id() * fat_width as u32 - cluster_in_fat_sector * self.bytes_per_sector as u32) as usize;
        let entry = unsafe { &data[index..index + fat_width].cast()[0] };
        Ok(entry)
    }
}

impl<'a> FileSystem for &'a Shared<VFat> {
    type File = File;
    type Dir = Dir;
    type Entry = Entry;

    fn open<P: AsRef<Path>>(self, path: P) -> io::Result<Self::Entry> {
        use traits::Entry;
        let mut dir = super::Entry::Dir(Dir::new_root(self));
        for component in path.as_ref().components() {
            match component {
                Component::Normal(name) => {
                    dir = dir.into_dir()
                        .ok_or(io::Error::new(io::ErrorKind::NotFound, ""))?
                        .find(name)?;
                }
                Component::ParentDir => {
                    dir = dir.into_dir()
                        .ok_or(io::Error::new(io::ErrorKind::NotFound, ""))?
                        .find("..")?;
                }
                _ => {}
            }
        }
        Ok(dir)
    }

    fn create_file<P: AsRef<Path>>(self, _path: P) -> io::Result<Self::File> {
        unimplemented!("read only file system")
    }

    fn create_dir<P>(self, _path: P, _parents: bool) -> io::Result<Self::Dir>
        where P: AsRef<Path>
    {
        unimplemented!("read only file system")
    }

    fn rename<P, Q>(self, _from: P, _to: Q) -> io::Result<()>
        where P: AsRef<Path>, Q: AsRef<Path>
    {
        unimplemented!("read only file system")
    }

    fn remove<P: AsRef<Path>>(self, _path: P, _children: bool) -> io::Result<()> {
        unimplemented!("read only file system")
    }
}
