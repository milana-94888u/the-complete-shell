mod parameter_expansion;
mod command_expansion;
mod arithmetic_expansion;

use crate::shell_structures::ShellToken;

#[derive(Clone, Debug)]
pub enum DollarSignExpansion {

}

impl ShellToken for DollarSignExpansion {
    fn restore_original(&self) -> Vec<u8> {
        //match self {  }
        vec![]
    }
}