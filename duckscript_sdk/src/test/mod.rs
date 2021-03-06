use crate::utils::state::{get_handles_sub_state, put_handle};
use duckscript::runner;
use duckscript::types::command::{Command, CommandResult, Commands};
use duckscript::types::error::ScriptError;
use duckscript::types::instruction::Instruction;
use duckscript::types::runtime::{Context, StateValue};
use std::collections::HashMap;

pub(crate) struct ErrorCommand {}

impl Command for ErrorCommand {
    fn name(&self) -> String {
        "test_error".to_string()
    }

    fn run(&self, _arguments: Vec<String>) -> CommandResult {
        CommandResult::Error("test".to_string())
    }
}

pub(crate) struct SetCommand {}

impl Command for SetCommand {
    fn name(&self) -> String {
        "test_set".to_string()
    }

    fn help(&self) -> String {
        "".to_string()
    }

    fn run(&self, arguments: Vec<String>) -> CommandResult {
        if arguments.is_empty() {
            CommandResult::Continue(None)
        } else {
            CommandResult::Continue(Some(arguments[0].clone()))
        }
    }
}

pub(crate) struct SetHandleCommand {}

impl Command for SetHandleCommand {
    fn name(&self) -> String {
        "test_set_handle".to_string()
    }

    fn requires_context(&self) -> bool {
        true
    }

    fn run_with_context(
        &self,
        arguments: Vec<String>,
        state: &mut HashMap<String, StateValue>,
        _variables: &mut HashMap<String, String>,
        _output_variable: Option<String>,
        _instructions: &Vec<Instruction>,
        _commands: &mut Commands,
        _line: usize,
    ) -> CommandResult {
        if arguments.is_empty() {
            CommandResult::Continue(None)
        } else {
            let state = get_handles_sub_state(state);
            state.insert(arguments[0].clone(), StateValue::String("test".to_string()));
            CommandResult::Continue(None)
        }
    }
}

pub(crate) struct ArrayCommand {}

impl Command for ArrayCommand {
    fn name(&self) -> String {
        "test_array".to_string()
    }

    fn requires_context(&self) -> bool {
        true
    }

    fn run_with_context(
        &self,
        arguments: Vec<String>,
        state: &mut HashMap<String, StateValue>,
        _variables: &mut HashMap<String, String>,
        _output_variable: Option<String>,
        _instructions: &Vec<Instruction>,
        _commands: &mut Commands,
        _line: usize,
    ) -> CommandResult {
        let mut array = vec![];

        for argument in arguments {
            array.push(StateValue::String(argument));
        }

        let key = put_handle(state, StateValue::List(array));

        CommandResult::Continue(Some(key))
    }
}

pub(crate) struct OnErrorCommand {}

impl Command for OnErrorCommand {
    fn name(&self) -> String {
        "on_error".to_string()
    }

    fn requires_context(&self) -> bool {
        true
    }

    fn run_with_context(
        &self,
        arguments: Vec<String>,
        _state: &mut HashMap<String, StateValue>,
        variables: &mut HashMap<String, String>,
        _output_variable: Option<String>,
        _instructions: &Vec<Instruction>,
        _commands: &mut Commands,
        _line: usize,
    ) -> CommandResult {
        let mut index = 0;
        for argument in arguments {
            index = index + 1;
            variables.insert(index.to_string(), argument.clone());
        }

        variables.insert("on_error_invoked".to_string(), "true".to_string());

        CommandResult::Continue(None)
    }
}

pub(crate) enum CommandValidation {
    None,
    Match(String, String),
    Contains(String, String),
    Any(String, Vec<String>),
}

pub(crate) fn test_common_command_functions(command: Box<dyn Command>) {
    assert!(command.name().len() > 0);
    command.help();
    command.aliases();
}

fn run_command(commands: Vec<Box<dyn Command>>, script: &str) -> Result<Context, ScriptError> {
    let mut context = Context::new();
    for command in commands {
        context.commands.set(command)?;
    }

    if !context.commands.exists("on_error") {
        let added = context.commands.set(Box::new(OnErrorCommand {}));
        assert!(added.is_ok());
    }

    runner::run_script(script, context)
}

pub(crate) fn run_script_and_crash(commands: Vec<Box<dyn Command>>, script: &str) {
    let result = run_command(commands, script);
    assert!(result.is_err());
}

pub(crate) fn run_script_and_error(
    commands: Vec<Box<dyn Command>>,
    script: &str,
    output_variable: &str,
) -> Context {
    let result = run_command(commands, script);
    match result {
        Ok(context) => {
            assert_eq!(
                context.variables.get(&output_variable.to_string()).unwrap(),
                "false"
            );
            assert_eq!(
                context
                    .variables
                    .get(&"on_error_invoked".to_string())
                    .unwrap(),
                "true"
            );

            context
        }
        Err(error) => panic!(error.to_string()),
    }
}

pub(crate) fn run_script_and_validate(
    commands: Vec<Box<dyn Command>>,
    script: &str,
    validation: CommandValidation,
) -> Context {
    let result = run_command(commands, script);
    match result {
        Ok(context) => {
            assert!(context
                .variables
                .get(&"on_error_invoked".to_string())
                .is_none());

            match validation {
                CommandValidation::None => assert!(context.variables.is_empty()),
                CommandValidation::Match(key, value) => {
                    assert!(!context.variables.is_empty());
                    assert_eq!(context.variables.get(&key), Some(&value))
                }
                CommandValidation::Contains(key, value) => {
                    assert!(!context.variables.is_empty());

                    let var_value = context.variables.get(&key).unwrap();
                    assert!(
                        var_value.contains(&value),
                        "The value: {} is not contained in: {}",
                        &value,
                        &var_value
                    )
                }
                CommandValidation::Any(key, values) => {
                    assert!(!context.variables.is_empty());

                    let var_value = context.variables.get(&key).unwrap();
                    assert!(
                        values.contains(&var_value),
                        "The value: {} is not contained in: {:#?}",
                        &var_value,
                        &values
                    )
                }
            };

            context
        }
        Err(error) => panic!(error.to_string()),
    }
}
