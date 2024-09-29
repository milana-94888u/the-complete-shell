pub mod shell_range;
pub mod shell_brace_list;

#[derive(Clone, Debug)]
pub enum BraceExpansion {
    Range(shell_range::ShellRange),
    List(shell_brace_list::ShellBraceList),
}
