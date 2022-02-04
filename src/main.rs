// First pass... a translation of the java version of TOY from the links below
// Goal: a web based interface to TOY - possibly a step up from "switches"
// see: https://introcs.cs.princeton.edu/java/62toy/
// see: https://introcs.cs.princeton.edu/java/64simulator/TOY.java.html
// program examples at: https://introcs.cs.princeton.edu/java/63programming/

mod machine;

use machine::machine::Machine;
use machine::external_env::external_env::ExternalEnv;
use machine::program_reader::program_reader::ProgramReader;
#[macro_use]
extern crate num_derive;


fn main() {
    let filename: &str = "program.txt";
    let mut reader = ProgramReader::new();
    reader.load_from_file(filename);
    let loads = reader.parse();
    let mut machine = Machine::new();
    let mut external = ExternalEnv::new(vec![25, 39]);
    machine.load(loads);
    // external.dump();
    // machine.dump_regs();
    // machine.dump_memory();
    machine.run(&mut external);
    // external.dump();
    // machine.dump_regs();
    // machine.dump_memory();
}
