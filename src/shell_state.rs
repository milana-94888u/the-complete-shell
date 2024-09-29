struct EnvVariable {
    name: Vec<u8>,
    value: Vec<Vec<u8>>,
    count: usize,
    is_array: bool,
}

pub struct ShellState {
    parameters: Vec<Vec<u8>>,

}