use std::collections::HashSet;

#[derive(Debug)]
enum Instruction {
    NOP,
    ACC,
    JMP
}
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

    let mut current_instruction: i32 = 0;
    let mut hash = HashSet::new();
    let mut acc = 0;
    loop {
        if hash.contains(&current_instruction) {println!("Acc is {}", acc); return;}
        let (next_op, delta) = input[current_instruction as usize].execute();
        acc += delta;
        hash.insert(current_instruction);
        current_instruction = next_op;
    }
}
