use common::memory::Memory;
use common::errors::EmulErrorKind;
use common::bus::*;

#[test]
fn write_out_of_bounds() {
    let mut m = Memory::new(100);
    let wr = m.writeByte(200, 3);
    match wr {
        Ok(_) => panic!("Write is ok"),
        Err(err) => assert_eq!(err.kind, EmulErrorKind::OutOfBounds)
    }
}

#[test]
fn read_out_of_bounds() {
    let mut memory = Memory::new(100);
    let rd = memory.readByte(150);
    match rd {
        Ok(_) => panic!("Read is ok"),
        Err(err) => assert_eq!(err.kind, EmulErrorKind::OutOfBounds)
    }
}

#[test]
fn write_and_read() {
    let mut memory = Memory::new(100);
    let data = 3u8;
    let addr = 55;
    let wr = memory.writeByte(addr, data);
    match wr {
        Ok(_) => {},
        Err(err) => panic!("Cannot write")
    }
    let rd = memory.readByte(addr);
    match rd {
        Ok(b) => assert_eq!(b, data),
        Err(err) => panic!("Cannot read")
    }
}

#[test]
fn write_block_out_of_bounds() {
    let mut memory = Memory::new(100);
    let data = vec![1u8; 5];
    let addr = 98;
    let wr = memory.writeBlock(addr, data);
    match wr {
        Ok(_) => panic!("Write block is successful"),
        Err(err) => assert_eq!(err.kind, EmulErrorKind::OutOfBounds)
    }
}

#[test]
fn read_and_write_block() {
    let mut memory = Memory::new(100);
    let data = vec![1u8; 5];
    let addr = 95;
    let wr = memory.writeBlock(addr, data);
    match wr {
        Ok(_) => {},
        Err(err) => panic!("Cannot write")
    }
}