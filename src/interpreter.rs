use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read, Write};

struct Lexer {
    code: Vec<char>,
    position_in_code: usize,
}

impl Lexer {
    pub fn new() -> Lexer {
        Lexer {
            code: Vec::new(),
            position_in_code: 0,
        }
    }

    pub fn fill(&mut self, code: &str) {
        for c in code.chars() {
            self.code.push(c);
        }
    }

    fn is_valid_instruction(&self, inst: char) -> bool {
        let valid = "><+-.,[]";
        if valid.contains(inst) {
            return true;
        } else {
            return false;
        }
    }

    pub fn next(&mut self) -> char {
        while self.position_in_code < self.code.len() && !self.is_valid_instruction(self.code[self.position_in_code]) {
            self.position_in_code += 1;
        }

        if self.position_in_code >= self.code.len() {
            return '@'; // EOF character.
        }

        let r = self.code[self.position_in_code];
        self.position_in_code += 1;
        return r;
    }
}

#[derive(Clone, Copy, PartialEq)]
enum IRInstructionKind {
    IncrementPointer,
    DecrementPointer,
    IncrementByte,
    DecrementByte,
    PrintByteAsChar,
    ReadInputToByte,
    JumpIfZero,
    JumpIfNotZero,
}

#[derive(Clone, Copy)]
struct IRInstruction {
    kind: IRInstructionKind,
    operand: Option<u8>,
}

const TOTAL_MEMORY_SIZE: usize = 100000; // 100000 cells.

pub struct Interpreter {
    instruction_pointer: usize,
    memory_pointer: usize,
    memory: [u8; TOTAL_MEMORY_SIZE],
    program: Vec<IRInstruction>,
    jump_map: HashMap<usize, usize>,
    lexer: Lexer,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            instruction_pointer: 0,
            memory_pointer: 0,
            memory: [0; TOTAL_MEMORY_SIZE],
            program: Vec::new(),
            jump_map: HashMap::new(),
            lexer: Lexer::new(),
        }
    }

    pub fn load_program(&mut self, program_path: &str) {
        let mut program_file = File::open(program_path).expect("[ERROR] Unable to open the program !");

        let mut program_buffer = String::new();

        program_file.read_to_string(&mut program_buffer).expect("[ERROR] Unable to read the program !");

        self.lexer.fill(program_buffer.as_str());

        let mut c = self.lexer.next();

        while c != '@' {
            let inst: IRInstruction;
            match c {
                '>' | '<' | '+' | '-' => {
                    let k: IRInstructionKind;
                    if c == '>' { k = IRInstructionKind::IncrementPointer; }
                    else if c == '<' { k = IRInstructionKind::DecrementPointer; }
                    else if c == '+' { k = IRInstructionKind::IncrementByte; }
                    else { k = IRInstructionKind::DecrementByte; }

                    let mut s = self.lexer.next();
                    let mut streak = 1u8;

                    while c == s {
                        streak += 1;
                        s = self.lexer.next();
                    }

                    inst = IRInstruction { kind: k, operand: Some(streak) };
                    c = s;
                },
                '.' | ',' | '[' | ']' => {
                    let k: IRInstructionKind;
                    if c == '.' { k = IRInstructionKind::PrintByteAsChar; }
                    else if c == ',' { k = IRInstructionKind::ReadInputToByte; }
                    else if c == '[' { k = IRInstructionKind::JumpIfZero; }
                    else { k = IRInstructionKind::JumpIfNotZero; }

                    inst = IRInstruction { kind: k, operand: None };
                    c = self.lexer.next();
                },
                _ => continue,
            }

            self.program.push(inst);
        }
    }

    fn precompute_jumps(&mut self) {
        let mut stack = Vec::new();

        let mut local_instruction_pointer = 0usize;

        while local_instruction_pointer < self.program.len() {
            let inst = self.program[local_instruction_pointer];

            match inst.kind {
                IRInstructionKind::JumpIfZero => stack.push(local_instruction_pointer),
                IRInstructionKind::JumpIfNotZero => {
                    let target = stack.pop().unwrap();
                    self.jump_map.insert(local_instruction_pointer, target);
                    self.jump_map.insert(target, local_instruction_pointer);
                },
                _ => (), // Other instructions aren't jump related.
            }

            local_instruction_pointer += 1;
        }
    }

    pub fn interpret(&mut self) {
        self.precompute_jumps();

        while self.instruction_pointer < self.program.len() {
            let inst = self.program[self.instruction_pointer];

            match inst.kind {
                IRInstructionKind::IncrementPointer => self.memory_pointer += inst.operand.unwrap() as usize,
                IRInstructionKind::DecrementPointer => self.memory_pointer -= inst.operand.unwrap() as usize,
                IRInstructionKind::IncrementByte => self.memory[self.memory_pointer] += inst.operand.unwrap(),
                IRInstructionKind::DecrementByte => self.memory[self.memory_pointer] -= inst.operand.unwrap(),
                IRInstructionKind::PrintByteAsChar => {
                    let byte_as_char = self.memory[self.memory_pointer] as char;
                    print!("{byte_as_char}");
                    io::stdout().flush().unwrap();
                },
                IRInstructionKind::ReadInputToByte => {
                    let mut input: [u8; 1] = [0; 1];
                    io::stdin().read_exact(&mut input).expect("[ERROR] Unable to read stdin !");
                    self.memory[self.memory_pointer] = input[0];
                },
                IRInstructionKind::JumpIfZero => {
                    if self.memory[self.memory_pointer] == 0 {
                        self.instruction_pointer = *self.jump_map.get(&self.instruction_pointer).unwrap();
                    }
                },
                IRInstructionKind::JumpIfNotZero => {
                    if self.memory[self.memory_pointer] != 0 {
                        self.instruction_pointer = *self.jump_map.get(&self.instruction_pointer).unwrap();
                    }
                }
            }

            self.instruction_pointer += 1;
        }
    }
}
