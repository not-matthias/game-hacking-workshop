use hax::ExternalMemory;
use hax::memlib::MemoryReadExt;
use env_logger::Env;

fn test_simple(mem: ExternalMemory) {
    // &value
    //
    let ref_value = 0x7ffc57418314_u64;
    let health = mem.read::<u32>(ref_value);

    log::info!("&value ({:x}) is pointing to {}", ref_value, health);

    // &&value
    //
    let ref_ref_value = 0x7ffc57418450;

    let value = mem.read::<u64>(ref_ref_value);
    log::info!("&&value ({:x}) is pointing to {:x}", ref_value, value);

    let health = mem.read::<u32>(value as u64);
    log::info!("&value ({:x}) is pointing to {:x}", value, health);
}

fn test_struct(mem: ExternalMemory) {
    let struct_ptr = 0x7ffd93a2ba40; // &value
    let value = mem.read::<u32>(struct_ptr);

    log::info!("struct_ptr ({:x}) is pointing to {}", struct_ptr, value);

    let armor = mem.read::<u32>(struct_ptr + 4);
    let money = mem.read::<u32>(struct_ptr + 8);

    log::info!("armor: {}", armor);
    log::info!("money: {}", money);
}


fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let pid = 1270336;
    let mem = ExternalMemory::new(pid);

    // test_simple(mem);
    test_struct(mem);
}
