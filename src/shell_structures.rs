pub mod shell_word;
pub mod shell_pipeline;
pub mod shell_list;
pub mod shell_command;
pub mod shell_variable_assignment;


pub trait ShellToken {
    fn restore_original(&self) -> Vec<u8>;
}