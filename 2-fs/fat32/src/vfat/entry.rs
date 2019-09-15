use traits;
use vfat::{Cluster, Dir, File, Metadata, Shared, VFat};

// TODO: You may need to change this definition.
#[derive(Debug)]
pub enum Entry {
    File(File),
    Dir(Dir)
}

// TODO: Implement any useful helper methods on `Entry`.

// FIXME: Implement `traits::Entry` for `Entry`.
impl traits::Entry for Entry {
    type File = File;
    type Dir = Dir;
    type Metadata = Metadata;

    fn name(&self) -> &str {
        match &self {
            Entry::File(file) => &file.name,
            Entry::Dir(dir) => &dir.name
        }
    }

    fn metadata(&self) -> &Self::Metadata {
        match &self {
            Entry::File(file) => &file.metadata,
            Entry::Dir(dir) => &dir.metadata
        }
    }

    fn as_file(&self) -> Option<&<Self as traits::Entry>::File> {
        match &self {
            Entry::File(file) => Some(file),
            _ => None
        }
    }

    fn as_dir(&self) -> Option<&<Self as traits::Entry>::Dir> {
        match &self {
            Entry::Dir(dir) => Some(dir),
            _ => None
        }
    }

    fn into_file(self) -> Option<<Self as traits::Entry>::File> {
        match self {
            Entry::File(file) => Some(file),
            _ => None
        }
    }

    fn into_dir(self) -> Option<<Self as traits::Entry>::Dir> {
        match self {
            Entry::Dir(dir) => Some(dir),
            _ => None
        }
    }
}
