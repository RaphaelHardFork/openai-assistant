mod config;

use derive_more::{Deref, From};
use serde::{Deserialize, Serialize};

use self::config::Config;
use crate::ais::asst::ThreadId;
use crate::ais::{asst::AsstId, OaClient};
use crate::utils::files::ensure_dir;
use crate::Result;
use std::path::PathBuf;

const AGENT_TOML: &str = "agent.toml";

#[derive(Debug)]
pub struct Agent {
    dir: PathBuf,
    oac: OaClient,
    asst_id: AsstId,
    config: Config,
}

#[derive(Debug, From, Deref, Deserialize, Serialize)]
pub struct Conv {
    thread_id: ThreadId,
}

// public
impl Agent {}

// private
impl Agent {
    fn data_dir(&self) -> Result<PathBuf> {
        let data_dir = self.dir.join(".agent");
        ensure_dir(&data_dir)?;
        Ok(data_dir)
    }

    fn data_files_dir(&self) -> Result<PathBuf> {
        let dir = self.data_dir()?.join("files");
        ensure_dir(&dir)?;
        Ok(dir)
    }
}
