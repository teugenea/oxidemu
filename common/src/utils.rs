use std::io::{BufReader, Read};
use std::fs::File;

pub fn load_rom(file_name: &str) -> Result<Vec<u8>, std::io::Error> {
    let mut input = BufReader::new(File::open(file_name).expect("Cannot open file file_name"));
    let mut target = vec![];
    loop {
        use std::io::ErrorKind;
        let mut buffer = [0u8; std::mem::size_of::<u8>()];
        let res = input.read_exact(&mut buffer);
        match res {
            Err(error) => {
                match error.kind() {
                    ErrorKind::UnexpectedEof => break,
                    _ => return Err(error)
                }
            }
            _ => {}
        }
        target.push(u8::from_le_bytes(buffer));
    }
    Ok(target)
}