use stack_vec::StackVec;
use console::{kprint, CONSOLE};
use std::str;
use std::path::{Path, PathBuf};
use FILE_SYSTEM;
use fat32::traits::{FileSystem, Dir as _Dir, Entry};
use fat32::vfat::Dir;
use std::io::{Read, Seek, SeekFrom};

/// Error type for `Command` parse failures.
#[derive(Debug)]
enum Error {
    Empty,
    TooManyArgs,
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
    let mut storage = [0u8; 512];
    let mut buf = StackVec::new(&mut storage);

    print_shell();
    let mut pwd = PathBuf::from("/");

    kprint!("initializing....\r\n");

    loop {
        kprint!("{} {}", pwd.to_str().unwrap(), prefix);
        read_command(&mut buf);
        execute_command(&mut buf, &mut pwd);
    }
}

fn read_command(mut buf: &mut StackVec<u8>) {
    loop {
        let input = CONSOLE.lock().read_byte();
        match input {
            8 | 127 => backspace(&mut buf),
            b'\r' | b'\n' => break,
            32..=126 => store_command(&mut buf, input),
            _ => ring_bell(),
        }
    }
}

fn print_shell() {
    new_line();
    kprint!("{}", "this is a super shell");
    new_line();
}

fn new_line() {
    kprint!("\r\n");
}

fn ring_bell() {
    kprint!("\u{7}");
}

fn backspace(buf: &mut StackVec<u8>) {
    if buf.is_empty() {
        ring_bell();
    } else {
        kprint!("\u{8} \u{8}");
        buf.pop();
    }
}

fn execute_command(buf: &mut StackVec<u8>, pwd: &mut PathBuf) {
    new_line();
    match Command::parse(str::from_utf8(buf.as_slice()).unwrap(), &mut [""; 64]) {
        Ok(input) => {
            let mut cmd = input.path();
            match cmd {
                "echo" => shell_echo(&input.args[1..]),
                "ls" => shell_ls(pwd),
                "cd" => shell_cd(pwd, &input.args[1..]),
                "pwd" => shell_pwd(pwd),
                "cat" => shell_cat(pwd, &input.args[1..]),
                _ => kprint!("unknown command: {}\r\n", cmd),
            }
        }
        Err(Error::TooManyArgs) => {
            kprint!("error: too many arguments\r\n");
        }
        _ => {}
    }

    buf.truncate(0);
}

fn store_command(buf: &mut StackVec<u8>, input: u8) {
    match buf.push(input) {
        Ok(_) => CONSOLE.lock().write_byte(input),
        Err(_) => ring_bell()
    }
}

fn shell_ls(pwd: &mut PathBuf) {
    let dir: Option<Dir> = FILE_SYSTEM.get().open_dir(pwd.as_path()).ok();
    let entries = dir.unwrap().entries().unwrap();
    for d in entries {
        if d.is_file() {
            kprint!("-");
        } else {
            kprint!("d");
        }
        kprint!("\t{}\r\n", d.name());
    }
}

fn shell_pwd(pwd: &mut PathBuf) {
    kprint!("{}\r\n", pwd.to_str().unwrap());
}

fn shell_echo(args: &[&str]) {
    for arg in args {
        kprint!("{} ", arg);
    }
    new_line();
}

fn shell_cd(pwd: &mut PathBuf, args: &[&str]) {
    let target = match args.len() {
        0 => "/",
        _ => args[0],
    };

    match target {
        ".." => { pwd.pop(); }
        "." => {}
        _ => {
            let mut new_dir = pwd.clone();
            new_dir.push(target);

            let dir = FILE_SYSTEM.get().open_dir(new_dir.as_path());
            match dir {
                Ok(_) => {
                    pwd.push(target);
                }
                Err(err) => kprint!("{}\r\n",err)
            }
        }
    }
}

fn shell_cat(pwd: &mut PathBuf, args: &[&str]) {
    for filename in args {
        let mut file = pwd.clone();
        file.push(filename);
        match FILE_SYSTEM.get().open_file(file.as_path()) {
            Ok(mut f) => {
                let mut offset = 0;
                loop {
                    f.seek(SeekFrom::Current(offset));
                    let mut buf = [0u8; 512];
                    let bytes_read = f.read(&mut buf).unwrap() as i64;
                    if bytes_read == 0 {
                        break;
                    } else {
                        offset += bytes_read;
                        kprint!("{}", String::from_utf8_lossy(&buf));
                    }
                }
            }
            Err(err) => kprint!("{}\r\n",err)
        }
    }
}
