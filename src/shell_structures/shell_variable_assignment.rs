use super::shell_word::ShellWord;

#[derive(Clone, Debug)]
pub enum VariableAssignmentType {
    Simple(ShellWord),
    List(Vec<ShellWord>)
}

#[derive(Clone, Debug)]
pub struct ShellVariableAssignment {
    pub identifier: Vec<u8>,
    pub value: VariableAssignmentType,
}
