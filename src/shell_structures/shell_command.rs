pub mod shell_simple_command;
pub mod shell_compound_command;
pub mod shell_coproc;
pub mod shell_function_definition;

use super::shell_word::ShellWord;

#[derive(Clone, Debug)]
pub enum ShellCommand {
    Simple(),
    Compound(),
    Coproc(),
    FunctionDefinition(),
}

#[derive(Clone, Debug)]
pub enum VariableAssignmentValue {
    Plain(ShellWord),
    List(Vec<ShellWord>),
}

#[derive(Clone, Debug)]
pub struct VariableAssignment {
    name: Vec<u8>,
    index: i64, // name[3]=val
    value: VariableAssignmentValue,
}
