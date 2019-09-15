#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone, Hash)]
pub struct Cluster(u32);

impl From<u32> for Cluster {
    fn from(raw_num: u32) -> Cluster {
        Cluster(raw_num & !(0xF << 28))
    }
}

// TODO: Implement any useful helper methods on `Cluster`.
impl Cluster {
    pub fn is_valid(&self) -> bool {
        self.0 >= 2
    }

    pub fn id(&self) -> u32 {
        self.0
    }

    pub fn index(&self) -> u32 {
        self.0 - 2
    }
}
