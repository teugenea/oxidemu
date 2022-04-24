use common::memory::Memory;
use common::errors::EmulErrorKind;
use common::bus::*;

#[test]
fn write_out_of_bounds() {
    let mut m = Memory::new(100);
    let wr = m.write_byte(200, 3);
    match wr {
        Ok(_) => panic!("Write is ok"),
        Err(err) => assert_eq!(err.kind, EmulErrorKind::OutOfBounds)
    }
}

#[test]
fn read_out_of_bounds() {
    let memory = Memory::new(100);
    let rd = memory.read_byte(150);
    match rd {
        Ok(_) => panic!("Read is successful but should not be"),
        Err(err) => assert_eq!(err.kind, EmulErrorKind::OutOfBounds)
    }
}

#[test]
fn read_and_write() {
    let mut memory = Memory::new(100);
    let data = 3u8;
    let addr = 55;
    let wr = memory.write_byte(addr, data);
    match wr {
        Ok(_) => {},
        Err(err) => panic!("Cannot write: {}", err)
    }
    let rd = memory.read_byte(addr);
    match rd {
        Ok(b) => assert_eq!(b, data),
        Err(err) => panic!("Cannot read: {}", err)
    }
}

/*
#[test]
fn write_block_out_of_bounds() {
    let mut memory = Memory::new(100);
    let data = vec![1u8; 5];
    let addr = 98;
    let wr = memory.write_block(addr, data);
    match wr {
        Ok(_) => panic!("Write block is successful but should not be"),
        Err(err) => assert_eq!(err.kind, EmulErrorKind::OutOfBounds)
    }
}

#[test]
fn read_and_write_block() {
    let mut memory = Memory::new(100);
    let data = vec![1u8; 5];
    let addr = 95;
    let wr = memory.write_block(addr, data);
    match wr {
        Ok(_) => {},
        Err(err) => panic!("Cannot write: {}", err)
    }
}
*/