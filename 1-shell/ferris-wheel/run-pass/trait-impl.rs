// FIXME: Make me pass! Diff budget: 25 lines.
#[derive(Debug)]
enum Duration {
    MilliSeconds(u64),
    Seconds(u32),
    Minutes(u16)
}

use Duration::MilliSeconds;
use Duration::Seconds;
use Duration::Minutes;

impl PartialEq for Duration {
    fn eq(&self, other: &Self) -> bool {
        let a = match self {
            &MilliSeconds(v) => v,
            &Seconds(v) => v as u64 * 1000,
            &Minutes(v) => v as u64 * 1000 * 60,
        };
        let b = match other {
            &MilliSeconds(v) => v,
            &Seconds(v) => v as u64 * 1000,
            &Minutes(v) => v as u64 * 1000 * 60,
        };
        a == b
    }
}

fn main() {
    assert_eq!(Seconds(120), Minutes(2));
    assert_eq!(Seconds(420), Minutes(7));
    assert_eq!(MilliSeconds(420000), Minutes(7));
    assert_eq!(MilliSeconds(43000), Seconds(43));
}
