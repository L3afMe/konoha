mod app;
mod client;
mod error;
mod fs;

#[tokio::main]
async fn main() {
    if let Err(why) = app::start_ui() {
        eprintln!("{}", why);
    }
}

