use env_logger::Env;

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    log::info!("Example Target Application");

    let health = 42;
    loop {
        log::info!("Value: {}", health);
        log::info!("&value: {:p}", &health);
        log::info!("&&value: {:p}", &&health);

        // Wait for user input
        //
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
    }
}
