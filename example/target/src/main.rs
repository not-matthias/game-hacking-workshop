use env_logger::Env;
use memoffset::offset_of;

#[derive(Debug, Copy, Clone)]
struct Player {
    health: u32,
    armor: u32,
    money: u32,
}

impl Player {
    pub fn new() -> Self {
        Self {
            health: 100,
            armor: 200,
            money: 1000,
        }
    }
}

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    log::info!("Example Target Application");
    log::info!("Pid: {}", std::process::id());

    let health: u32 = 42;
    let player = Player::new();
    loop {
        log::info!("health: {}", health);
        log::info!("&health: {:p}", &health);
        println!();

        log::info!("player: {:?}", player);
        log::info!("&player: {:p}", &player);
        println!();

        log::info!("player.health: {}", player.health);
        log::info!("&player.health: {:p}", &player.health);
        log::info!("offset: {:x}", offset_of!(Player, health));
        println!();

        log::info!("player.armor: {}", player.armor);
        log::info!("&player.armor: {:p}", &player.armor);
        log::info!("offset: {:x}", offset_of!(Player, armor));
        println!();

        log::info!("player.money: {}", player.money);
        log::info!("&player.money: {:p}", &player.money);
        log::info!("offset: {:x}", offset_of!(Player, money));
        println!();

        // Wait for user input
        //
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
    }
}
