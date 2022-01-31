pub mod program_reader{
    use crate::machine::machine::Instruction;
    use std::{
        fs::File,
        io::{BufRead, BufReader},
        path::Path,
    };

    pub struct ProgramReader {
        lines: Vec<String>,
    }

    impl ProgramReader {
        pub fn new() -> Self{
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
        pub(crate) fn load_from_vec(&mut self, lines: Vec<String>) {
            self.lines = lines;
        }
        pub fn parse(&self) -> Vec<Instruction> {
            self.lines.iter().map(|line| self.parse_line(line)).collect()
        }
        fn parse_line(&self, line: &String) -> Instruction {
            println!("in parse -- {:?}",line);
            let dummy =
                Instruction::new(0,0, 0, 0, 0);
            dummy
        }
    }
}



#[cfg(test)]
mod program_reader_tests {
    #[test]
    fn it_works(){
        assert_eq!(2, 1+1);
    }
}