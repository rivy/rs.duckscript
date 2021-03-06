mod assert;
mod assert_eq;
mod assert_fail;

use crate::utils::pckg;
use duckscript::types::command::Commands;
use duckscript::types::error::ScriptError;

static PACKAGE: &str = "test";

pub(crate) fn load(commands: &mut Commands, parent: &str) -> Result<(), ScriptError> {
    let package = pckg::concat(parent, PACKAGE);

    commands.set(assert::create(&package))?;
    commands.set(assert_eq::create(&package))?;
    commands.set(assert_fail::create(&package))?;

    Ok(())
}
