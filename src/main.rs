// region:			--- Modules

mod agent;
mod ais;
mod error;
mod utils;

use agent::Agent;
use serde_json::to_string;

use crate::ais::{
    asst::{self, CreateConfig},
    new_oa_client,
};

pub use self::error::{Error, Result};

// endregion:		--- Modules

#[tokio::main]
async fn main() {
    println!();

    match start().await {
        Ok(_) => println!("\nBye!\n"),
        Err(e) => println!("\nError: {}", e),
    }
}

const DEFAULT_DIR: &str = "agent";

// region:			--- Types
/// input command from the user
enum Cmd {
    Quit,
    Chat(String),
    RefreshAll,
    RefreshConv,
    RefreshInst,
    RefreshFiles,
}

impl Cmd {
    fn from_input(input: impl Into<String>) -> Self {
        let input: &str = &input.into();

        match input {
            "/q" => Self::Quit,
            "/r" | "/ra" => Self::RefreshAll,
            "/ri" => Self::RefreshInst,
            "/rf" => Self::RefreshFiles,
            "/rc" => Self::RefreshConv,
            chat => Self::Chat(chat.to_string()),
        }
    }
}
// endregion:		--- Types

async fn start() -> Result<()> {
    let mut agent = Agent::init_from_dir(DEFAULT_DIR, false).await?;
    let mut conv = agent.load_or_create_conv(false).await?;

    println!("->> agent {} - conv {conv:?}", agent.name());

    Ok(())
}
