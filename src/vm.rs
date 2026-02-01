use crate::code;
use crate::compiler::Compiler;
use crate::object::Object;

const STACK_SIZE: usize = 2048;
const GLOBALS_SIZE: usize = 65536; // Max 65k globals

pub struct VM {
    constants: Vec<Object>,
    instructions: code::Instructions,
    
    stack: Vec<Object>,
    sp: usize, // Stack Pointer
    
    pub globals: Vec<Object>, // Global Storage
}

impl VM {
    pub fn new(bytecode: Compiler) -> VM {
        VM {
            constants: bytecode.constants,
            instructions: bytecode.instructions,
            stack: vec![Object::Null; STACK_SIZE],
            sp: 0,
            globals: vec![Object::Null; GLOBALS_SIZE],
        }
    }

    pub fn stack_top(&self) -> Option<&Object> {
        if self.sp == 0 { return None; }
        Some(&self.stack[self.sp - 1])
    }

    pub fn run(&mut self) -> Result<(), String> {
        let mut ip = 0; // Instruction Pointer

        while ip < self.instructions.len() {
            let op = self.instructions[ip];
            ip += 1;

            match op {
                code::OP_CONSTANT => {
                    let const_index = u16::from_be_bytes([self.instructions[ip], self.instructions[ip+1]]) as usize;
                    ip += 2;
                    let obj = self.constants[const_index].clone();
                    self.push(obj)?;
                },
                code::OP_POP => {
                    self.pop();
                },
                
                // --- ARITHMETIC ---
                code::OP_ADD => {
                    let right = self.pop();
                    let left = self.pop();
                    let result = self.execute_binary_operation(left, right)?;
                    self.push(result)?;
                },
                
                // --- LOGIC ---
                code::OP_TRUE => self.push(Object::Boolean(true))?,
                code::OP_FALSE => self.push(Object::Boolean(false))?,
                code::OP_EQUAL => {
                    let right = self.pop();
                    let left = self.pop();
                    self.push(Object::Boolean(left == right))?;
                },
                code::OP_NOT_EQUAL => {
                    let right = self.pop();
                    let left = self.pop();
                    self.push(Object::Boolean(left != right))?;
                },
                code::OP_GREATER_THAN => {
                    let right = self.pop();
                    let left = self.pop();
                    match (left, right) {
                        (Object::Integer(l), Object::Integer(r)) => self.push(Object::Boolean(l > r))?,
                        _ => return Err("Type mismatch for >".to_string()),
                    }
                },

                // --- JUMPS ---
                code::OP_JUMP => {
                    let pos = u16::from_be_bytes([self.instructions[ip], self.instructions[ip+1]]) as usize;
                    ip = pos;
                },
                code::OP_JUMP_NOT_TRUTHY => {
                    let pos = u16::from_be_bytes([self.instructions[ip], self.instructions[ip+1]]) as usize;
                    ip += 2;
                    let condition = self.pop();
                    if !self.is_truthy(condition) {
                        ip = pos;
                    }
                },

                // --- GLOBALS ---
                code::OP_SET_GLOBAL => {
                    let global_index = u16::from_be_bytes([self.instructions[ip], self.instructions[ip+1]]) as usize;
                    ip += 2;
                    let val = self.pop();
                    self.globals[global_index] = val;
                },
                code::OP_GET_GLOBAL => {
                    let global_index = u16::from_be_bytes([self.instructions[ip], self.instructions[ip+1]]) as usize;
                    ip += 2;
                    let val = self.globals[global_index].clone();
                    self.push(val)?;
                },

                _ => return Err(format!("Unknown Opcode: {}", op)),
            }
        }
        Ok(())
    }

    // --- HELPERS ---

    fn execute_binary_operation(&self, left: Object, right: Object) -> Result<Object, String> {
        match (left, right) {
            (Object::Integer(l), Object::Integer(r)) => Ok(Object::Integer(l + r)),
            _ => Err("Type mismatch or unsupported operation".to_string()),
        }
    }

    fn push(&mut self, obj: Object) -> Result<(), String> {
        if self.sp >= STACK_SIZE {
            return Err("Stack Overflow".to_string());
        }
        self.stack[self.sp] = obj;
        self.sp += 1;
        Ok(())
    }

    fn pop(&mut self) -> Object {
        if self.sp == 0 { return Object::Null; }
        self.sp -= 1;
        self.stack[self.sp].clone()
    }

    fn is_truthy(&self, obj: Object) -> bool {
        match obj {
            Object::Boolean(b) => b,
            Object::Null => false,
            _ => true,
        }
    }
}