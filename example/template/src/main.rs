#![allow(unused)]

use env_logger::Env;
use hax::memlib::MemoryReadExt;
use hax::memlib::MemoryWriteExt;
use hax::ExternalMemory;

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let pid = 1291683;
    let mem = ExternalMemory::new(pid);

    // TODO: Our code goes here
}
