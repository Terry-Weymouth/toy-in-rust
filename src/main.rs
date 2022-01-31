// First pass... a translation of the java version of TOY from the links below
// Goal: a web based interface to TOY - possibly a step up from "switches"
//      setting by "typing" ??
// see: https://introcs.cs.princeton.edu/java/62toy/
// see: https://introcs.cs.princeton.edu/java/64simulator/TOY.java.html
// program examples at: https://introcs.cs.princeton.edu/java/63programming/

mod machine;

use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

// ---

fn main() {
    let lines = lines_from_file("program.txt").expect("Could not load lines");
    for line in lines {
        println!("{:?}", line);
    }
}
