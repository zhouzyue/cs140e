use stack_vec::StackVec;
use console::{kprint, kprintln, CONSOLE};
use std::str;

/// Error type for `Command` parse failures.
#[derive(Debug)]
enum Error {
    Empty,
    TooManyArgs
}

/// A structure representing a single shell command.
struct Command<'a> {
    args: StackVec<'a, &'a str>
}

impl<'a> Command<'a> {
    /// Parse a command from a string `s` using `buf` as storage for the
    /// arguments.
    ///
    /// # Errors
    ///
    /// If `s` contains no arguments, returns `Error::Empty`. If there are more
    /// arguments than `buf` can hold, returns `Error::TooManyArgs`.
    fn parse(s: &'a str, buf: &'a mut [&'a str]) -> Result<Command<'a>, Error> {
        let mut args = StackVec::new(buf);
        for arg in s.split(' ').filter(|a| !a.is_empty()) {
            args.push(arg).map_err(|_| Error::TooManyArgs)?;
        }

        if args.is_empty() {
            return Err(Error::Empty);
        }

        Ok(Command { args })
    }

    /// Returns this command's path. This is equivalent to the first argument.
    fn path(&self) -> &str {
        self.args[0]
    }
}

/// Starts a shell using `prefix` as the prefix for each line. This function
/// never returns: it is perpetually in a shell loop.
pub fn shell(prefix: &str) -> ! {
    let mut buf_storage = [0u8; 1024];
    let mut buf = StackVec::new(&mut buf_storage);
    loop {
        let mut console = CONSOLE.lock();
        let byte = console.read_byte();
        if byte == 8 || byte == 127 {
            kprint!(" ");
            console.write_byte(byte);
        } else if byte == b'\r' || byte == b'\n' {
            kprintln!("");
//            buf.push(byte);
            let mut commands = [""; 64];
            Command::parse(str::from_utf8(buf.as_slice()).unwrap(), &mut commands);
            kprintln!("{}", prefix);
        } else {
            console.write_byte(byte);
        }
    }
}
