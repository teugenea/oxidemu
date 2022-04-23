use common::memory::Memory;
use common::errors::EmulErrorKind;
use common::bus::*;

#[test]
fn read_and_write_bus() {
    let mut bus = Bus::new();
    let memory = Memory::new(100);
    bus.add_dev(DeviceType::Memory, Box::new(memory));
    let res = bus.write(DeviceType::Memory, 12, 42u8);
}