mod config;

use self::config::Config;
use crate::ais::asst::{self, ThreadId};
use crate::ais::new_oa_client;
use crate::ais::{asst::AsstId, OaClient};
use crate::utils::cli::ico_check;
use crate::utils::files::{
    bundle_to_file, ensure_dir, list_files, load_from_json, load_from_toml, read_to_string,
    save_to_json,
};
use crate::Result;
use derive_more::{Deref, From};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

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
impl Agent {
    pub fn name(&self) -> &str {
        &self.config.name
    }

    pub async fn init_from_dir(dir: impl AsRef<Path>, recreate_asst: bool) -> Result<Self> {
        let dir = dir.as_ref();

        // load from the dir
        let config: Config = load_from_toml(dir.join(AGENT_TOML))?;

        // Get or create agent
        let oac = new_oa_client()?;
        let asst_id = asst::load_or_create(&oac, (&config).into(), recreate_asst).await?;

        // create agent
        let agent = Agent {
            dir: dir.to_path_buf(),
            oac,
            asst_id,
            config,
        };

        // upload instruction
        agent.upload_instructions().await?;

        // upload files
        agent.upload_files(false).await?;

        Ok(agent)
    }

    pub async fn upload_instructions(&self) -> Result<bool> {
        let file = self.dir.join(&self.config.instructions_file);
        if file.exists() {
            let inst_content = read_to_string(&file)?;
            asst::upload_instructions(&self.oac, &self.asst_id, inst_content).await?;
            println!("{} Instructions uploaded", ico_check());
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub async fn upload_files(&self, recreate: bool) -> Result<u32> {
        let mut num_uploaded = 0;

        // the .agent/files
        let data_files_dir = self.data_files_dir()?;

        // clean the .agent/files
        let exclude_element = format!("*{}*", &self.asst_id);
        for file in list_files(
            &data_files_dir,
            Some(&["*.rs", "*.md"]),
            Some(&[&exclude_element]),
        )? {
            let file_str = file.to_string_lossy();
            // safeguard
            if !file_str.contains(".agent") {
                return Err(format!("Should not be deleted: '{}'", file_str).into());
            }
            fs::remove_file(&file)?;
        }

        // generate and upload .agent/files bundle files.
        for bundle in self.config.file_bundles.iter() {
            let src_dir = self.dir.join(&bundle.src_dir);

            if src_dir.is_dir() {
                let src_globs: Vec<&str> = bundle.src_globs.iter().map(AsRef::as_ref).collect();
                let files = list_files(&src_dir, Some(&src_globs), None)?;

                if !files.is_empty() {
                    // compute bundle file name
                    let bundle_file_name = format!(
                        "{}-{}-bundle-{}.{}",
                        self.name(),
                        bundle.bundle_name,
                        self.asst_id,
                        bundle.dst_ext
                    );
                    let bundle_file = self.data_files_dir()?.join(bundle_file_name);

                    // if does not exist, force reupload
                    let force_reupload = recreate || !bundle_file.exists();

                    // Rebundle not matter exist or not
                    bundle_to_file(files, &bundle_file)?;

                    // upload
                    let (_, uploaded) = asst::upload_file_by_name(
                        &self.oac,
                        &self.asst_id,
                        &bundle_file,
                        force_reupload,
                    )
                    .await?;

                    if uploaded {
                        num_uploaded += 1;
                    }
                }
            }
        }

        Ok(num_uploaded)
    }

    pub async fn load_or_create_conv(&self, recreate: bool) -> Result<Conv> {
        let conv_file = self.data_dir()?.join("conv.json");

        if recreate && conv_file.exists() {
            fs::remove_file(&conv_file)?;
        }

        let conv = if let Ok(conv) = load_from_json::<Conv>(&conv_file) {
            asst::get_thread(&self.oac, &conv.thread_id)
                .await
                .map_err(|_| format!("Cannot find thread_id for {:?}", conv))?;
            println!("{} Conversation loaded", ico_check());
            conv
        } else {
            let thread_id = asst::create_thread(&self.oac).await?;
            println!("{} Conversation created", ico_check());
            let conv = thread_id.into();
            save_to_json(&conv_file, &conv)?;
            conv
        };

        Ok(conv)
    }

    pub async fn chat(&self, conv: &Conv, msg: &str) -> Result<String> {
        asst::run_thread_msg(&self.oac, &self.asst_id, &conv.thread_id, msg).await
    }
}

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
