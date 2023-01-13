#![allow(unused)]

use env_logger::Env;
use hax::memlib::MemoryReadExt;
use hax::memlib::MemoryWriteExt;
use hax::ExternalMemory;

fn test_simple(mem: ExternalMemory) {
    // read
    //
    let ref_value = 0x7ffc57418314_u64;
    let health = mem.read::<u32>(ref_value);

    log::info!("&value ({:x}) is pointing to {}", ref_value, health);

    // write
    //
    mem.write::<u32>(ref_value, &1337);
}

fn test_struct(mem: ExternalMemory) {
    let struct_ptr = 0x7ffc7f744a10; // see &player
    let value = mem.read::<u32>(struct_ptr);

    log::info!("struct_ptr ({:x}) is pointing to {}", struct_ptr, value);

    let armor = mem.read::<u32>(struct_ptr + 4);
    let money = mem.read::<u32>(struct_ptr + 8);

    log::info!("armor: {}", armor);
    log::info!("money: {}", money);

    // Write
    //
    mem.write::<u32>(struct_ptr + 4, &1337);
    mem.write::<u32>(struct_ptr + 8, &1337);
}

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let pid = 1291683;
    let mem = ExternalMemory::new(pid);

    // test_simple(mem);
    test_struct(mem);
}
