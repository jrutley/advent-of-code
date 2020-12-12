use std::collections::HashSet;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Instruction {
    NOP,
    ACC,
    JMP
}

#[derive(Copy, Clone)]
struct Op {
    position: i32,
    instruction: Instruction,
    value: i32
}
impl Op {
    fn execute(&self) -> (i32, i32) {
        match self.instruction {
            Instruction::NOP => (self.position + 1, 0),
            Instruction::ACC => (self.position + 1, self.value),
            Instruction::JMP => (self.position + self.value, 0)
        }
    }
}

fn main() {
    let file_data = textfilereader::read_file_by_line("input.txt");
    let input = generate_initial_instructions(file_data);

    let mut twiddle_counter = 0;
    while twiddle_counter < input.len() {
        let new_instruction = match input[twiddle_counter].instruction {
            Instruction::NOP => Instruction::JMP,
            Instruction::ACC => Instruction::ACC,
            Instruction::JMP => Instruction::NOP
        };
        if new_instruction == input[twiddle_counter].instruction {
            twiddle_counter += 1;
            continue;
        }
        // twiddle input
        let mut twiddled_input: Vec<Op> = input.iter().map(|&i| i).collect();
        twiddled_input[twiddle_counter].instruction = new_instruction;

        let (success, acc) = run_simulation(&twiddled_input);

        if success {
            println!("Acc is {}", acc);
            return;
        }
        twiddle_counter += 1;
    }
    panic!("Couldn't find the instruction!");
}

fn run_simulation(input: &Vec<Op>) -> (bool, i32) {
    let mut current_instruction: i32 = 0;
    let mut hash = HashSet::new();
    let mut acc = 0;
    let input_len = input.len() as i32;
    loop {
        if current_instruction == input_len {return (true, acc);}
        if hash.contains(&current_instruction) {return (false, acc);}
        let (next_op, delta) = input[current_instruction as usize].execute();
        acc += delta;
        hash.insert(current_instruction);
        current_instruction = next_op;
    }
}

fn generate_initial_instructions(file_data: Vec<String>) -> Vec<Op> {
    let mut counter = 0;
    let input: Vec<Op> = file_data.iter().map(|f| {
        let position = counter;
        let input: Vec<&str> = f.split(' ').collect();
        let op: Instruction = match input[0] {
            "nop" => Instruction::NOP,
            "acc" => Instruction::ACC,
            "jmp" => Instruction::JMP,
            _ => panic!("Invalid input")
        };
        counter += 1;
        return Op {
            position: position,
            instruction: op,
            value: input[1].parse::<i32>().unwrap()
        }
    }).collect();
    input
}