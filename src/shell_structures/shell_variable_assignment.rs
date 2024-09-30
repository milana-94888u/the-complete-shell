use super::shell_word::ShellWord;

pub enum VariableAssignmentType {
    Simple(ShellWord),
    List(Vec<ShellWord>)
}

pub struct ShellVariableAssignment {
    pub identifier: Vec<u8>,
    pub value: VariableAssignmentType,
}
