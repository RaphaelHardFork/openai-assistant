// region:			--- Modules

mod agent;
mod ais;
mod error;
mod utils;

pub use self::error::{Error, Result};

use crate::ais::asst::{self, CreateConfig};
use crate::ais::new_oa_client;
use crate::utils::cli::{ico_res, prompt, txt_res};
use agent::Agent;
use serde_json::to_string;
use textwrap::wrap;

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
#[derive(Debug)]
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

    loop {
        println!("");
        let input = prompt("Posez une question")?;
        let cmd = Cmd::from_input(input);

        match cmd {
            Cmd::Quit => break,
            Cmd::Chat(msg) => {
                let res = agent.chat(&conv, &msg).await?;
                let res = wrap(&res, 80).join("\n");
                println!("{} {}", ico_res(), txt_res(res));
            }
            Cmd::RefreshAll => {
                agent = Agent::init_from_dir(DEFAULT_DIR, true).await?;
                conv = agent.load_or_create_conv(true).await?;
            }
            Cmd::RefreshConv => {
                conv = agent.load_or_create_conv(true).await?;
            }
            Cmd::RefreshInst => {
                agent.upload_instructions().await?;
                conv = agent.load_or_create_conv(true).await?;
            }
            Cmd::RefreshFiles => {
                agent.upload_files(true).await?;
                conv = agent.load_or_create_conv(true).await?;
            }
            other => println!("{:?} command not supported", other),
        }
    }

    println!("->> agent {} - conv {conv:?}", agent.name());

    Ok(())
}
