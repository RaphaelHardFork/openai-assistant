use crate::Result;
use console::{style, Style, StyledObject};
use dialoguer::theme::{self, ColorfulTheme};
use dialoguer::Input;

// region:			--- Prompt

pub fn prompt(text: &str) -> Result<String> {
    let theme = ColorfulTheme {
        prompt_style: Style::new().for_stderr().color256(45),
        prompt_prefix: style("?".to_string()).color256(45).for_stderr(),
        ..Default::default()
    };

    let input = Input::with_theme(&theme);
    let res = input.with_prompt(text).interact_text()?;

    Ok(res)
}

// endregion:		--- Prompt

// region:			--- Icons
// https://www.compart.com/fr/unicode/html

pub fn ico_check() -> StyledObject<&'static str> {
    style("✔").green()
}
pub fn ico_uploading() -> StyledObject<&'static str> {
    style("↥").yellow()
}
pub fn ico_uploaded() -> StyledObject<&'static str> {
    style("↥").green()
}
pub fn ico_deleted_ok() -> StyledObject<&'static str> {
    style("⌫").green()
}
pub fn ico_err() -> StyledObject<&'static str> {
    style("⚠").red()
}
pub fn ico_res() -> StyledObject<&'static str> {
    style("֍").color256(45)
}
pub fn ico_res_1() -> StyledObject<&'static str> {
    style("⏵").color256(45)
}

// endregion:		--- Icons

// region:			--- Text output

pub fn txt_res(text: String) -> StyledObject<String> {
    style(text).bright()
}

// endregion:		--- Text output
