mod basename;
mod canonical;
mod cp;
mod dirname;
mod list;
mod mkdir;
mod mv;
mod print;
mod read;
mod rm;
mod rmdir;
mod touch;
mod write;

use crate::utils::pckg;
use duckscript::types::command::Commands;
use duckscript::types::error::ScriptError;

static PACKAGE: &str = "fs";

pub(crate) fn load(commands: &mut Commands, parent: &str) -> Result<(), ScriptError> {
    let package = pckg::concat(parent, PACKAGE);

    commands.set(basename::create(&package))?;
    commands.set(canonical::create(&package))?;
    commands.set(cp::create(&package))?;
    commands.set(dirname::create(&package))?;
    commands.set(list::create(&package))?;
    commands.set(mkdir::create(&package))?;
    commands.set(mv::create(&package))?;
    commands.set(print::create(&package))?;
    commands.set(read::create(&package))?;
    commands.set(rm::create(&package))?;
    commands.set(rmdir::create(&package))?;
    commands.set(touch::create(&package))?;
    commands.set(write::create(&package))?;

    Ok(())
}
