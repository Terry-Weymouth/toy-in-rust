pub mod external_env {
    #[derive(Debug)]
    pub struct ExternalEnv {
        input: Vec<u16>,
        output: Vec<u16>,
        pub input_for_dump: String,
        pub output_for_dump: String
    }

    impl ExternalEnv {
        pub fn new(input: Vec<u16>) -> Self {
            let output = vec![];
            let mut str = String::from("");
            for item in &input {
                if str.len() == 0 {
                    str = format!("Input {}({:04X}x)", item, item);
                } else {
                    str = format!("{}, {}({:04X}x)", str, item, item)
                }
            }
            if str.len() == 0 {
                str = String::from("No input")
            }
            Self {
                input,
                output,
                input_for_dump: String::from(str),
                output_for_dump: String::from(""),
            }
        }
        pub(crate) fn is_end_of_input(&self) -> bool{
            self.input.len() == 0
        }
        pub(crate) fn get_next_word(&mut self) -> Option<u16> {
            if self.input.len() == 0 {
                return None
            }
            let value = self.input.remove(0);
            Option::from(value)
        }
        pub(crate) fn put_word(&mut self, word: u16) {
            if self.output_for_dump.len() == 0 {
                self.output_for_dump = format!("Output: {}({:04X}x)", word, word)
            } else {
                self.output_for_dump = format!("{}, {}({:04X}x)", self.output_for_dump, word, word)
            }
            self.output.push(word)
        }
        pub(crate) fn peek_at_last_output(&self) -> u16 {
            let last = self.output.len() - 1;
            self.output[last]
        }
        pub(crate) fn dump(&self){
            println!("External environment for i/o");
            println!("  {}", self.input_for_dump);
            if self.output.len() == 0{
                println!("  No output")
            } else {
                println!("  {}", self.output_for_dump);
            }
        }
    }
}

#[cfg(test)]
mod external_env_tests {
    use super::external_env::ExternalEnv;

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
        assert_eq!(env.input_for_dump, "Input 4660(1234x), 9029(2345x), 13398(3456x)");
        test_read_next_word(&mut env, 0x1234);
        test_read_next_word(&mut env, 0x2345);
        test_read_next_word(&mut env, 0x3456);
    }
    #[test]
    fn end_of_input_file(){
        let mut env = ExternalEnv::new(vec![0x1234]);
        assert!(!env.is_end_of_input());
        test_read_next_word(&mut env, 0x1234);
        assert!(env.is_end_of_input());
    }
    #[test]
    fn dump_no_input() {
        let env = ExternalEnv::new(vec![]);
        assert_eq!(env.input_for_dump, "No input");
    }
    #[test]
    fn write_to_env() {
        let mut env = ExternalEnv::new(vec![]);
        for word in vec![0x1234, 0x2345, 0x3456] {
            env.put_word(word);
            assert_eq!(word, env.peek_at_last_output())
        }
        assert_eq!(env.output_for_dump, "Output: 4660(1234x), 9029(2345x), 13398(3456x)");
    }
}
