use crate::ast;
use crate::code;
use crate::object::Object;
use crate::symbol_table::SymbolTable;

pub struct Compiler {
    pub instructions: code::Instructions,
    pub constants: Vec<Object>,
    pub symbol_table: SymbolTable,
    
    // Tracking for "pop" removal (to make blocks return values like expressions)
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
            symbol_table: SymbolTable::new(),
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
            ast::Statement::Let { name, value } => {
                // 1. Compile value (pushes result to stack)
                self.compile_expression(value)?;
                // 2. Define symbol and get index
                let symbol = self.symbol_table.define(name);
                // 3. Emit SetGlobal
                self.emit(code::OP_SET_GLOBAL, vec![symbol.index]);
            },
            ast::Statement::Expression(exp) => {
                self.compile_expression(exp)?;
                // Statement expressions pop their result to keep stack clean
                //self.emit(code::OP_POP, vec![]); 
            },
            _ => return Err("Statement type not implemented yet".to_string()),
        }
        Ok(())
    }

    fn compile_expression(&mut self, exp: ast::Expression) -> Result<(), String> {
        match exp {
            ast::Expression::Infix { left, operator, right } => {
                // Special Case: Swap < to >
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
            
            // --- VARIABLES ---
            ast::Expression::Identifier(name) => {
                if let Some(symbol) = self.symbol_table.resolve(&name) {
                    self.emit(code::OP_GET_GLOBAL, vec![symbol.index]);
                } else {
                    return Err(format!("Undefined variable: {}", name));
                }
            },

            // --- IF / ELSE ---
            ast::Expression::If { condition, consequence, alternative } => {
                self.compile_expression(*condition)?;

                // Emit JumpNotTruthy with dummy 9999
                let jump_not_truthy_pos = self.emit(code::OP_JUMP_NOT_TRUTHY, vec![9999]);

                self.compile_block(consequence)?;
                if self.last_instruction_is_pop() { self.remove_last_pop(); }

                // Emit Jump with dummy 9999
                let jump_pos = self.emit(code::OP_JUMP, vec![9999]);

                // Patch NotTruthy
                let after_consequence_pos = self.instructions.len();
                self.change_operand(jump_not_truthy_pos, after_consequence_pos);

                if let Some(alt) = alternative {
                    self.compile_block(alt)?;
                    if self.last_instruction_is_pop() { self.remove_last_pop(); }
                } else {
                    // Else-less ifs return Null
                    let null_idx = self.add_constant(Object::Null);
                    self.emit(code::OP_CONSTANT, vec![null_idx]);
                }

                // Patch Jump
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

    // --- HELPERS ---

    pub fn add_constant(&mut self, obj: Object) -> usize {
        self.constants.push(obj);
        self.constants.len() - 1
    }

    pub fn emit(&mut self, op: code::Opcode, operands: Vec<usize>) -> usize {
        let ins = code::make(op, operands);
        let pos = self.instructions.len();
        self.instructions.extend(ins);
        
        self.previous_instruction = self.last_instruction;
        self.last_instruction = Some(EmittedInstruction { opcode: op, position: pos });
        
        pos
    }

    fn change_operand(&mut self, op_pos: usize, operand: usize) {
        let op = self.instructions[op_pos];
        let new_instruction = code::make(op, vec![operand]);
        for (i, byte) in new_instruction.iter().enumerate() {
            self.instructions[op_pos + i] = *byte;
        }
    }

    fn last_instruction_is_pop(&self) -> bool {
        match self.last_instruction {
            Some(ins) => ins.opcode == code::OP_POP,
            None => false,
        }
    }

    fn remove_last_pop(&mut self) {
        if let Some(ins) = self.last_instruction {
            self.instructions.truncate(ins.position);
            self.last_instruction = self.previous_instruction;
        }
    }
}