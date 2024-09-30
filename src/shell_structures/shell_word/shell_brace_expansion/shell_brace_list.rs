use crate::shell_structures::shell_word::ShellWord;
use crate::shell_structures::ShellToken;

#[derive(Clone, Debug)]
pub struct ShellBraceList {
    words: Vec<ShellWord>,
}

impl ShellToken for ShellBraceList {
    fn restore_original(&self) -> Vec<u8> {
        let mut result = vec![b'{'];
        for (i, word) in self.words.iter().enumerate() {
            if i > 0 {
                result.push(b',');
            }
            result.extend(word.restore_original());
        }
        result.push(b'}');
        result
    }
}