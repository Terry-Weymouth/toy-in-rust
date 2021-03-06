pub mod program_reader {
    use crate::machine::ProgramLoadWord;
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::path::Path;
    use regex::Regex;

    pub struct ProgramReader {
        lines: Vec<String>,
    }

    impl ProgramReader {
        pub fn new() -> Self {
            let lines: Vec<String> = vec![];
            Self {
                lines,
            }
        }
        fn lines_from_file(&mut self, filename: impl AsRef<Path>) {
            let file = File::open(filename).expect("no such file");
            let buf = BufReader::new(file);
            self.lines = buf.lines()
                .map(|l| l.expect("Could not parse line"))
                .collect();
        }
        pub fn load_from_file(&mut self, filename: impl AsRef<Path>) {
            let _ = &self.lines_from_file(filename);
        }
        pub fn load_from_vec(&mut self, lines: Vec<String>) {
            self.lines = lines;
        }
        pub fn parse(&self) -> Vec<ProgramLoadWord> {
            self.lines.iter()
                .filter_map(|line| self.parse_line(line))
                .collect()
        }
        fn parse_line(&self, line: &String) -> Option<ProgramLoadWord> {
            let re = Regex::new(r"^([[:xdigit:]]{2}): *([[:xdigit:]]{4})").unwrap();
            let flag = re.is_match(line);
            if !flag {
                None
            } else {
                let cap = re.captures(line).unwrap();
                let address_string = cap.get(1).map_or("", |m| m.as_str());
                let content_string = cap.get(2).map_or("", |m| m.as_str());
                let address = u8::from_str_radix(address_string, 16).unwrap();
                let content = u16::from_str_radix(content_string, 16).unwrap();
                Some(ProgramLoadWord::new(address, content))
            }
        }
    }

    #[cfg(test)]
    mod program_reader_tests {
        use super::*;

        #[test]
        fn basic_reader_test() {
            let test_program_strings = vec![
                "10: 8AFF",   // read to R[A]                  a = StdIn.readInt();
                "11: 8BFF",   // read to R[B]                  b = StdIn.readInt();
            ];
            let mut program_text: Vec<String> = vec![];
            for s in test_program_strings {
                program_text.push(String::from(s));
            }
            let mut reader = ProgramReader::new();
            reader.load_from_vec(program_text);
            let loads = reader.parse();
            assert_eq!(2, loads.len());
        }
    }
}