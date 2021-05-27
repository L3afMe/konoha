use error::Result;

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

async fn run() -> Result<()> {
    fs::create_directories()?;

    app::start_app()
}

