use std::io::{BufReader, Read};
use std::fs::File;

pub fn load_rom(file_name: &str, start_addr: usize, target: &mut Vec<u8>) {
    let mut input = BufReader::new(File::open(file_name).expect("Cannot open file file_name"));
    let mut i = start_addr;
    loop {
        use std::io::ErrorKind;
        let mut buffer = [0u8; std::mem::size_of::<u8>()];
        let res = input.read_exact(&mut buffer);
        match res {
            Err(error) if error.kind() == ErrorKind::UnexpectedEof => break,
            _ => {}
        }
        res.expect("error during read");
        target[i] = u8::from_le_bytes(buffer);
        i += 1;
    }
}