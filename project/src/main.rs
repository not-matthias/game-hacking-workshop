#![allow(unused)]

use env_logger::Env;
use hax::memlib::MemoryReadExt;
use hax::ExternalMemory;
use hax::memlib::MemoryWriteExt;


#[cfg(target_os = "linux")]
const PLAYER_POINTER: u64 = 0x5F0E10;
#[cfg(target_os = "linux")]
const HEALTH: u64 = 0x100;
#[cfg(target_os = "linux")]
const ARMOR: u64 = 0x104;


#[cfg(target_os = "windows")]
const PLAYER_POINTER: u64 = 0x10F4F4;
#[cfg(target_os = "windows")]
const HEALTH: u64 = 0xF8;
#[cfg(target_os = "windows")]
const ARMOR: u64 = 0x150;

struct Player {
    mem: ExternalMemory,
    player: u64
}

impl Player {
    pub fn new(mem: ExternalMemory) -> Self {
        let player = mem.read::<u32>(PLAYER_POINTER) as u64;

        Self {
            mem,
            player
        }
    }

    pub fn health(&self) -> u16 {
        self.mem.read(self.player + HEALTH)
    }

    pub fn set_health(&self, health: u16)  {
        self.mem.write(self.player + HEALTH, &health)
    }

    pub fn armor(&self) -> u16 {
        self.mem.read(self.player + ARMOR)
    }

    pub fn set_armor(&self, armor: u16) {
        self.mem.write(self.player + ARMOR, &armor)
    }
}

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let pid = 1284855;
    let mem = ExternalMemory::new(pid);

    let player = Player::new(mem);
    log::info!("health: {}", player.health());
    log::info!("armor: {}", player.armor());

    player.set_health(1337);
    player.set_armor(1337);
}
