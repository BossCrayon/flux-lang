use std::fmt;

// 1. Define the Opcodes (The "Assembly Language" of Flux)
// We use simple bytes (u8) to represent instructions.
pub type Instructions = Vec<u8>;
pub type Opcode = u8;

// Enum-like constants for our instructions
pub const OP_CONSTANT: Opcode = 0;
pub const OP_ADD: Opcode      = 1;
pub const OP_POP: Opcode      = 2;
pub const OP_TRUE: Opcode     = 3;
pub const OP_FALSE: Opcode    = 4;
pub const OP_EQUAL: Opcode    = 5;
pub const OP_NOT_EQUAL: Opcode = 6;
pub const OP_GREATER_THAN: Opcode = 7;
pub const OP_JUMP_NOT_TRUTHY: Opcode = 8;
pub const OP_JUMP: Opcode = 9;
pub const OP_GET_GLOBAL: Opcode = 10;
pub const OP_SET_GLOBAL: Opcode = 11;
// 2. Definition Struct (Helper to understand operands)
// e.g., OP_CONSTANT needs 2 extra bytes to store the index of the constant.
pub struct Definition {
    pub name: String,
    pub operand_widths: Vec<usize>,
}

pub fn lookup(op: u8) -> Option<Definition> {
    match op {
        OP_CONSTANT => Some(Definition { name: "OpConstant".to_string(), operand_widths: vec![2] }),
        OP_ADD      => Some(Definition { name: "OpAdd".to_string(), operand_widths: vec![] }),
        OP_POP      => Some(Definition { name: "OpPop".to_string(), operand_widths: vec![] }),
        // NEW:
        OP_TRUE     => Some(Definition { name: "OpTrue".to_string(), operand_widths: vec![] }),
        OP_FALSE    => Some(Definition { name: "OpFalse".to_string(), operand_widths: vec![] }),
        OP_EQUAL    => Some(Definition { name: "OpEqual".to_string(), operand_widths: vec![] }),
        OP_NOT_EQUAL=> Some(Definition { name: "OpNotEqual".to_string(), operand_widths: vec![] }),
        OP_GREATER_THAN => Some(Definition { name: "OpGreaterThan".to_string(), operand_widths: vec![] }),
        OP_JUMP_NOT_TRUTHY => Some(Definition { name: "OpJumpNotTruthy".to_string(), operand_widths: vec![2] }),
        OP_JUMP            => Some(Definition { name: "OpJump".to_string(), operand_widths: vec![2] }),
        OP_GET_GLOBAL => Some(Definition { name: "OpGetGlobal".to_string(), operand_widths: vec![2] }),
        OP_SET_GLOBAL => Some(Definition { name: "OpSetGlobal".to_string(), operand_widths: vec![2] }),
        _ => None,
    }
}

// 3. Make Function: Compiles an Opcode + Operands into Bytes
pub fn make(op: Opcode, operands: Vec<usize>) -> Instructions {
    let def = match lookup(op) {
        Some(d) => d,
        None => return vec![],
    };

    let mut instruction_len = 1;
    for w in &def.operand_widths { instruction_len += w; }

    let mut instruction = vec![0; instruction_len];
    instruction[0] = op;

    let mut offset = 1;
    for (i, &o) in operands.iter().enumerate() {
        let width = def.operand_widths[i];
        match width {
            2 => {
                // Write u16 (Big Endian)
                instruction[offset] = ((o >> 8) & 0xFF) as u8;
                instruction[offset + 1] = (o & 0xFF) as u8;
            },
            _ => {},
        }
        offset += width;
    }
    instruction
}

// 4. Disassembler (For Debugging) - Turns bytes back into text
pub fn print_instructions(ins: &Instructions) -> String {
    let mut out = String::new();
    let mut i = 0;
    while i < ins.len() {
        let def = match lookup(ins[i]) {
            Some(d) => d,
            None => {
                out.push_str(&format!("ERROR: Unknown Opcode {}\n", ins[i]));
                i += 1;
                continue;
            }
        };

        let (operands, read) = read_operands(&def, &ins[i+1..]);
        out.push_str(&format!("{:04} {}\n", i, fmt_instruction(&def, &operands)));
        i += 1 + read;
    }
    out
}

fn read_operands(def: &Definition, ins: &[u8]) -> (Vec<usize>, usize) {
    let mut operands = vec![];
    let mut offset = 0;
    for width in &def.operand_widths {
        match width {
            2 => {
                let val = read_u16(&ins[offset..]);
                operands.push(val);
            },
            _ => {},
        }
        offset += *width;
    }
    (operands, offset)
}

fn fmt_instruction(def: &Definition, operands: &[usize]) -> String {
    let operand_count = def.operand_widths.len();
    match operand_count {
        0 => def.name.clone(),
        1 => format!("{} {}", def.name, operands[0]),
        _ => format!("ERROR: Unhandled operand count for {}", def.name),
    }
}

fn read_u16(ins: &[u8]) -> usize {
    ((ins[0] as usize) << 8) | (ins[1] as usize)
}