pub mod shell_range;
pub mod shell_brace_list;

use crate::shell_structures::ShellToken;

#[derive(Clone, Debug)]
pub enum BraceExpansion {
    Range(shell_range::ShellRange),
    List(shell_brace_list::ShellBraceList),
}


impl ShellToken for BraceExpansion {
    fn restore_original(&self) -> Vec<u8> {
        match self {
            BraceExpansion::Range(r) => r.restore_original(),
            BraceExpansion::List(l) => l.restore_original(),
        }
    }
} // wrong, no additional parameters for range
