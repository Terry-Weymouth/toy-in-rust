#[derive(Debug)]
pub struct ExternalEnv {
    input: Vec<u16>,
    output: Vec<u16>,
}

impl ExternalEnv{
    pub(crate) fn new(input: Vec<u16>) -> Self {
        let output = vec![];
        Self {
            input,
            output
        }
    }
    pub(crate) fn get_next_word(&mut self) -> Option<u16> {
        if self.input.len() == 0 {
            return None
        }
        let value = self.input.remove(0);
        Option::from(value)
    }
    pub(crate) fn put_word(&mut self, word: u16) {
        self.output.push(word)
    }
    pub(crate) fn peek_at_last_output(&self) -> u16{
        let last = self.output.len() - 1;
        self.output[last]
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    mod read_from_external {
        use super::*;

        fn test_read_next_word(env: &mut ExternalEnv, expected: u16) {
            let opt_value = env.get_next_word();
            let mut word: u16 = 0;
            match opt_value
            {
                Some(value) => {
                    word = value;
                },
                None => {
                    assert!(false)
                }
            }
            assert_eq!(expected, word);
        }

        #[test]
        fn read_from_env() {
            let mut env = ExternalEnv::new(vec![0x1234, 0x2345, 0x3456]);
            test_read_next_word(&mut env, 0x1234);
            test_read_next_word(&mut env, 0x2345);
            test_read_next_word(&mut env, 0x3456);
        }
        #[test]
        fn end_of_input_file(){
            // test detection of EOF for read todo!()
        }
        #[test]
        fn write_to_env() {
            let mut env = ExternalEnv::new(vec![]);
            for word in vec![0x1234, 0x2345, 0x3456] {
                env.put_word(word);
                assert_eq!(word, env.peek_at_last_output())
            }
        }
    }
}
