use crate::utils::{io, pckg};
use duckscript::types::command::{Command, CommandResult};

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

struct CommandImpl {
    package: String,
}

impl Command for CommandImpl {
    fn name(&self) -> String {
        pckg::concat(&self.package, "GetCanonicalPath")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["canonicalize".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn run(&self, arguments: Vec<String>) -> CommandResult {
        if arguments.is_empty() {
            CommandResult::Error("Path not provided.".to_string())
        } else {
            let path = io::get_canonical_path(&arguments[0]);

            CommandResult::Continue(Some(path.to_string()))
        }
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
