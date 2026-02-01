use crate::ast;
use crate::code;
use crate::object::Object;

pub struct Compiler {
    pub instructions: code::Instructions,
    pub constants: Vec<Object>,
    
    // Tracking for "pop" removal (to make blocks return values)
    last_instruction: Option<EmittedInstruction>,
    previous_instruction: Option<EmittedInstruction>,
}

#[derive(Clone, Copy)]
struct EmittedInstruction {
    opcode: code::Opcode,
    position: usize,
}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler {
            instructions: vec![],
            constants: vec![],
            last_instruction: None,
            previous_instruction: None,
        }
    }

    pub fn compile(&mut self, program: Vec<ast::Statement>) -> Result<(), String> {
        for stmt in program {
            self.compile_statement(stmt)?;
        }
        Ok(())
    }

    fn compile_statement(&mut self, stmt: ast::Statement) -> Result<(), String> {
        match stmt {
            ast::Statement::Expression(exp) => {
                self.compile_expression(exp)?;
               // self.emit(code::OP_POP, vec![]); 
            },
            // We'll add Let/Return here later
            _ => return Err("Statement type not implemented yet".to_string()),
        }
        Ok(())
    }

    fn compile_expression(&mut self, exp: ast::Expression) -> Result<(), String> {
        match exp {
            ast::Expression::Infix { left, operator, right } => {
                // Rewire LessThan (<) to GreaterThan (>)
                if operator == "<" {
                    self.compile_expression(*right)?;
                    self.compile_expression(*left)?;
                    self.emit(code::OP_GREATER_THAN, vec![]);
                    return Ok(());
                }

                self.compile_expression(*left)?;
                self.compile_expression(*right)?;
                
                match operator.as_str() {
                    "+" => { self.emit(code::OP_ADD, vec![]); },
                    "==" => { self.emit(code::OP_EQUAL, vec![]); },
                    "!=" => { self.emit(code::OP_NOT_EQUAL, vec![]); },
                    ">"  => { self.emit(code::OP_GREATER_THAN, vec![]); },
                    _ => return Err(format!("Unknown operator: {}", operator)),
                };
            },
            ast::Expression::IntegerLiteral(value) => {
                let integer = Object::Integer(value);
                let const_index = self.add_constant(integer); 
                self.emit(code::OP_CONSTANT, vec![const_index]);
            },
            ast::Expression::Boolean(true)  => { self.emit(code::OP_TRUE, vec![]); },
            ast::Expression::Boolean(false) => { self.emit(code::OP_FALSE, vec![]); },
            
            // --- IF / ELSE LOGIC ---
            ast::Expression::If { condition, consequence, alternative } => {
                // 1. Compile Condition
                self.compile_expression(*condition)?;

                // 2. Emit JumpNotTruthy with dummy 9999
                let jump_not_truthy_pos = self.emit(code::OP_JUMP_NOT_TRUTHY, vec![9999]);

                // 3. Compile Consequence
                self.compile_block(consequence)?;
                
                // If the block ended with a Pop, remove it so the value stays on stack
                if self.last_instruction_is_pop() {
                    self.remove_last_pop();
                }

                // 4. Emit Jump (to skip else) with dummy 9999
                let jump_pos = self.emit(code::OP_JUMP, vec![9999]);

                // 5. PATCH JumpNotTruthy: Make it jump to HERE (after consequence)
                let after_consequence_pos = self.instructions.len();
                self.change_operand(jump_not_truthy_pos, after_consequence_pos);

                // 6. Compile Alternative (Else)
                if let Some(alt) = alternative {
                    self.compile_block(alt)?;
                    if self.last_instruction_is_pop() {
                        self.remove_last_pop();
                    }
                } else {
                    // If no else, return Null
                    let null_idx = self.add_constant(Object::Null);
                    self.emit(code::OP_CONSTANT, vec![null_idx]);
                }

                // 7. PATCH Jump: Make it jump to HERE (end of everything)
                let after_alternative_pos = self.instructions.len();
                self.change_operand(jump_pos, after_alternative_pos);
            },
            _ => return Err("Expression type not implemented yet".to_string()),
        }
        Ok(())
    }

    fn compile_block(&mut self, block: ast::BlockStatement) -> Result<(), String> {
        for stmt in block.statements {
            self.compile_statement(stmt)?;
        }
        Ok(())
    }

    // --- Helpers ---

    pub fn add_constant(&mut self, obj: Object) -> usize {
        self.constants.push(obj);
        self.constants.len() - 1
    }

    pub fn emit(&mut self, op: code::Opcode, operands: Vec<usize>) -> usize {
        let ins = code::make(op, operands);
        let pos = self.instructions.len();
        self.instructions.extend(ins);
        
        // Track instruction for Pop removal
        self.previous_instruction = self.last_instruction;
        self.last_instruction = Some(EmittedInstruction { opcode: op, position: pos });
        
        pos
    }

    // Back-Patching: Overwrite an operand at a specific position
    fn change_operand(&mut self, op_pos: usize, operand: usize) {
        let op = self.instructions[op_pos];
        let new_instruction = code::make(op, vec![operand]);
        
        for (i, byte) in new_instruction.iter().enumerate() {
            self.instructions[op_pos + i] = *byte;
        }
    }

    // Check if the last thing we wrote was "Pop"
    fn last_instruction_is_pop(&self) -> bool {
        match self.last_instruction {
            Some(ins) => ins.opcode == code::OP_POP,
            None => false,
        }
    }

    // Undo the last "Pop" (physically remove bytes from vector)
    fn remove_last_pop(&mut self) {
        if let Some(ins) = self.last_instruction {
            self.instructions.truncate(ins.position);
            self.last_instruction = self.previous_instruction;
        }
    }
}