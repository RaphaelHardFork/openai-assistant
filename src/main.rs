// region:			--- Modules

mod agent;
mod ais;
mod error;
mod utils;

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

async fn start() -> Result<()> {
    let oac = new_oa_client()?;

    let asst_config = CreateConfig {
        name: "agent-01".to_string(),
        model: "gpt-3.5-turbo-1106".to_string(),
    };

    let asst_id = asst::load_or_create(&oac, asst_config, false).await?;
    asst::upload_instruction(
        &oac,
        &asst_id,
        r#"
    Tu es un super développeur, soit concis dans tes réponses.

    Si on te demande quel est le meilleur langage, réponds Rust et de loin.
    Et le deuxième meilleur, réponds COBOL.
    "#
        .to_string(),
    )
    .await?;

    let thread_id = asst::create_thread(&oac).await?;
    let msg = asst::run_thread_msg(
        &oac,
        &asst_id,
        &thread_id,
        "Quelle est le meilleur langage?",
    )
    .await?;

    println!("->> asst_id: {asst_id}");

    Ok(())
}
