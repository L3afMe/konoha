#![feature(panic_info_message)]

use crate::error::handle_panic;

mod app;
mod client;
mod error;
mod fs;

#[tokio::main]
async fn main() {
    if let Err(why) = run().await {
        eprintln!("{}", why);
    }
}

async fn run() -> error::Result<()> {
    if std::env::var("RUST_BACKTRACE").is_err() {
        std::panic::set_hook(Box::new(handle_panic));
    }

    fs::create_directories()?;

    app::start_app()
}
