// First pass... a translation of the java version of TOY from the links below
// Goal: a web based interface to TOY - possibly a step up from "switches"
//      setting by "typing" ??
// see: https://introcs.cs.princeton.edu/java/62toy/
// see: https://introcs.cs.princeton.edu/java/64simulator/TOY.java.html

struct Machine {
    memory: [u16; 256],
}
impl Machine {
    fn new() -> Self {
        let memory: [u16; 256] = [0; 256];
        Self {
            memory
        }
    }
    fn set_memory_to_value(&mut self, index: usize, value: u16) {
        assert!(index < 256);
        println!{"memory inside {:x}", self.memory[index]};
        self.memory[index] = value;
        println!{"memory inside {:x}", self.memory[index]};
    }
}

struct ExternalSource{
    input: Vec<u16>,
}
impl ExternalSource{
    fn new(input: Vec<u16>) -> Self {
        Self {
            input
        }
    }
    fn get_next_word(&mut self) -> Option<u16> {
        if self.input.len() == 0 {
            return None
        }
        let value = self.input.remove(0);
        Option::from(value)
    }
}

fn main() {
    let mut machine = Machine::new();
    let mut env = ExternalSource::new(vec![0x1234, 0x2345, 0x3456]);
    let index = 5;
    let opt_value = env.get_next_word();
    match opt_value
    {
        Some(value) => {
            println!("memory before {:x}", machine.memory[index]);
            machine.set_memory_to_value(index, value);
            println!("memory after {:x}", machine.memory[index]);
        },
        None =>{
        }
    };
}


