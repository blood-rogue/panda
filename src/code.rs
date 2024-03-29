use std::{fmt::Write, vec::from_elem};

use num_enum::TryFromPrimitive;
use strum::{Display, EnumIter};

pub type Instructions = Vec<u8>;

#[derive(
    Clone, Copy, Display, EnumIter, PartialEq, Eq, PartialOrd, Ord, Debug, TryFromPrimitive,
)]
#[repr(u8)]
pub enum Opcode {
    Constant,
    Pop,
    PopNoRet,
    Dup,

    // Infix binary operators
    Add,
    Sub,
    Mul,
    Div,
    Mod,

    // Infix bitwise operators
    BitXor,
    BitAnd,
    BitOr,
    Shr,
    Shl,

    // Keyword literals
    True,
    False,
    Nil,

    // Infix comparison operators
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanEqual,

    // Prefix operators
    Minus,
    Bang,

    // Infix boolean operators
    And,
    Or,

    // Conditional Jumps
    Jump,
    JumpNotTruthy,

    // Bindings to names
    GetGlobal,
    SetGlobal,

    // Complex Literal
    Array,
    Dict,
    Index,
    Range,

    // Function Opcodes
    ReturnValue,
    Call,
    Return,
    GetLocal,
    SetLocal,
    GetBuiltin,
    Closure,
    GetFree,
    CurrentClosure,
    Method,

    Scope,
    Constructor,
    ClassMember,
    Delete,

    // Iterator
    Next,
    Start,
    JumpEnd,

    // Method Name
    String,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Definition {
    name: &'static str,
    operand_widths: &'static [usize],
}

const DEFINITIONS: &[Definition] = &[
    Definition {
        name: "Constant",
        operand_widths: &[2],
    },
    Definition {
        name: "Pop",
        operand_widths: &[],
    },
    Definition {
        name: "PopNoRet",
        operand_widths: &[],
    },
    Definition {
        name: "Dup",
        operand_widths: &[],
    },
    Definition {
        name: "Add",
        operand_widths: &[],
    },
    Definition {
        name: "Sub",
        operand_widths: &[],
    },
    Definition {
        name: "Mul",
        operand_widths: &[],
    },
    Definition {
        name: "Div",
        operand_widths: &[],
    },
    Definition {
        name: "Mod",
        operand_widths: &[],
    },
    Definition {
        name: "BitXor",
        operand_widths: &[],
    },
    Definition {
        name: "BitAnd",
        operand_widths: &[],
    },
    Definition {
        name: "BitOr",
        operand_widths: &[],
    },
    Definition {
        name: "Shr",
        operand_widths: &[],
    },
    Definition {
        name: "Shl",
        operand_widths: &[],
    },
    Definition {
        name: "True",
        operand_widths: &[],
    },
    Definition {
        name: "False",
        operand_widths: &[],
    },
    Definition {
        name: "Nil",
        operand_widths: &[],
    },
    Definition {
        name: "Equal",
        operand_widths: &[],
    },
    Definition {
        name: "NotEqual",
        operand_widths: &[],
    },
    Definition {
        name: "GreaterThan",
        operand_widths: &[],
    },
    Definition {
        name: "GreaterThanEqual",
        operand_widths: &[],
    },
    Definition {
        name: "Minus",
        operand_widths: &[],
    },
    Definition {
        name: "Bang",
        operand_widths: &[],
    },
    Definition {
        name: "And",
        operand_widths: &[],
    },
    Definition {
        name: "Or",
        operand_widths: &[],
    },
    Definition {
        name: "Jump",
        operand_widths: &[2],
    },
    Definition {
        name: "JumpNotTruthy",
        operand_widths: &[2],
    },
    Definition {
        name: "GetGlobal",
        operand_widths: &[2],
    },
    Definition {
        name: "SetGlobal",
        operand_widths: &[2],
    },
    Definition {
        name: "Array",
        operand_widths: &[2],
    },
    Definition {
        name: "Dict",
        operand_widths: &[2],
    },
    Definition {
        name: "Index",
        operand_widths: &[],
    },
    Definition {
        name: "Range",
        operand_widths: &[1],
    },
    Definition {
        name: "ReturnValue",
        operand_widths: &[],
    },
    Definition {
        name: "Call",
        operand_widths: &[1],
    },
    Definition {
        name: "Return",
        operand_widths: &[],
    },
    Definition {
        name: "GetLocal",
        operand_widths: &[1],
    },
    Definition {
        name: "SetLocal",
        operand_widths: &[1],
    },
    Definition {
        name: "GetBuiltin",
        operand_widths: &[1],
    },
    Definition {
        name: "Closure",
        operand_widths: &[2, 1],
    },
    Definition {
        name: "GetFree",
        operand_widths: &[1],
    },
    Definition {
        name: "CurrentClosure",
        operand_widths: &[],
    },
    Definition {
        name: "Method",
        operand_widths: &[8, 1, 1],
    },
    Definition {
        name: "Scope",
        operand_widths: &[1],
    },
    Definition {
        name: "Constructor",
        operand_widths: &[1],
    },
    Definition {
        name: "ClassMember",
        operand_widths: &[8, 1],
    },
    Definition {
        name: "Delete",
        operand_widths: &[2],
    },
    Definition {
        name: "Next",
        operand_widths: &[],
    },
    Definition {
        name: "Start",
        operand_widths: &[],
    },
    Definition {
        name: "JumpEnd",
        operand_widths: &[2, 2],
    },
    Definition {
        name: "String",
        operand_widths: &[1],
    },
];

pub fn make(op: Opcode, operands: &[usize]) -> Instructions {
    let Some(def) = DEFINITIONS.get(op as usize) else {
        return Vec::new();
    };

    let mut instruction_len = 1;
    for w in def.operand_widths {
        instruction_len += *w;
    }

    let mut instruction = from_elem(0, instruction_len);
    instruction[0] = op as u8;

    let mut offset = 1;
    for (i, o) in operands.iter().enumerate() {
        let width = def.operand_widths[i];
        match width {
            1 => {
                instruction[offset] = u8::try_from(*o).unwrap();
            }

            2 => {
                instruction = [
                    &instruction[..offset],
                    &u16::try_from(*o).unwrap().to_be_bytes(),
                    &instruction[offset + 2..],
                ]
                .concat();
            }

            8 => {
                instruction = [
                    &instruction[..offset],
                    &o.to_be_bytes(),
                    &instruction[offset + 8..],
                ]
                .concat();
            }

            _ => {}
        }

        offset += width;
    }

    instruction
}

pub fn read_u64(ins: &[u8], offset: usize) -> usize {
    let u: [u8; 8] = ins[offset..offset + 8].try_into().unwrap();

    u64::from_be_bytes(u) as usize
}

pub fn read_u16(ins: &[u8], offset: usize) -> usize {
    let u: [u8; 2] = ins[offset..offset + 2].try_into().unwrap();

    u16::from_be_bytes(u) as usize
}

pub const fn read_u8(ins: &[u8], offset: usize) -> usize {
    ins[offset] as usize
}

pub const fn read_bool(ins: &[u8], offset: usize) -> bool {
    ins[offset] != 0
}

pub fn lookup_definition(op: u8) -> Result<Definition, String> {
    DEFINITIONS.get(op as usize).map_or_else(
        || Err(format!("opcode {op} undefined")),
        |def| Ok(def.clone()),
    )
}

pub fn instructions_to_string(ins: &[u8]) -> String {
    let mut out = String::new();
    out.push('\n');

    let mut i = 0;

    while i < ins.len() {
        let def = match lookup_definition(ins[i]) {
            Ok(def) => def,
            Err(err) => {
                writeln!(out, "ERROR: {err}").unwrap();
                continue;
            }
        };

        let (operands, read) = read_operands(&def, &ins[i + 1..]);
        writeln!(out, "{:04}  {}", i, fmt_instruction(&def, &operands)).unwrap();

        i += read + 1;
    }

    out
}

fn fmt_instruction(def: &Definition, operands: &[usize]) -> String {
    let operand_count = def.operand_widths.len();

    if operands.len() != operand_count {
        return format!(
            "ERROR: operand len {} does not match defined {}\n",
            operands.len(),
            operand_count
        );
    }

    match operand_count {
        0 => def.name.to_string(),
        1 => format!("{:<16} {:>5}", def.name, operands[0]),
        2 => format!("{:<16} {:>5} {:>5}", def.name, operands[0], operands[1]),
        3 => format!(
            "{:<16} {:>5} {:>5} {:>5}",
            def.name, operands[0], operands[1], operands[2]
        ),
        _ => format!("ERROR: unhandled operand_count for {}\n", def.name),
    }
}

pub fn read_operands(def: &Definition, ins: &[u8]) -> (Vec<usize>, usize) {
    let mut operands = std::vec::from_elem(0, def.operand_widths.len());
    let mut offset = 0;

    for (i, width) in def.operand_widths.iter().enumerate() {
        match *width {
            1 => operands[i] = read_u8(ins, offset),
            2 => operands[i] = read_u16(ins, offset),
            8 => operands[i] = read_u64(ins, offset),

            _ => {}
        }

        offset += *width;
    }

    (operands, offset)
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use strum::IntoEnumIterator;

    use super::*;

    struct MakeTestCase {
        op: Opcode,
        operands: Vec<usize>,
        expected: Vec<u8>,
    }

    #[test]
    fn test_make() {
        let test_cases = [
            MakeTestCase {
                op: Opcode::Constant,
                operands: Vec::from([65534]),
                expected: Vec::from([Opcode::Constant as u8, 255, 254]),
            },
            MakeTestCase {
                op: Opcode::Add,
                operands: Vec::new(),
                expected: Vec::from([Opcode::Add as u8]),
            },
            MakeTestCase {
                op: Opcode::GetLocal,
                operands: Vec::from([255]),
                expected: Vec::from([Opcode::GetLocal as u8, 255]),
            },
            MakeTestCase {
                op: Opcode::Closure,
                operands: Vec::from([65534, 255]),
                expected: Vec::from([Opcode::Closure as u8, 255, 254, 255]),
            },
        ];

        for test_case in test_cases {
            let instruction = make(test_case.op, &test_case.operands);

            assert_eq!(instruction.len(), test_case.expected.len());

            for (i, &b) in test_case.expected.iter().enumerate() {
                assert_eq!(instruction[i], b);
            }
        }
    }

    #[test]
    fn test_instructions_string() {
        let instructions = Vec::from([
            make(Opcode::Add, &[]),
            make(Opcode::GetLocal, &[1]),
            make(Opcode::Constant, &[2]),
            make(Opcode::Constant, &[65535]),
            make(Opcode::Closure, &[65535, 255]),
        ]);

        let expected = "
0000  Add
0001  GetLocal             1
0003  Constant             2
0006  Constant         65535
0009  Closure          65535   255
";

        let concatted = instructions.concat();

        assert_eq!(instructions_to_string(&concatted), expected);
    }

    #[test]
    fn test_read_operands() {
        let test_cases = [
            (Opcode::Constant, Vec::from([65535]), 2),
            (Opcode::Add, Vec::new(), 0),
            (Opcode::GetLocal, Vec::from([255]), 1),
            (Opcode::Closure, Vec::from([65535, 255]), 3),
        ];

        for (op, operands, bytes_read) in test_cases {
            let instruction = make(op, &operands);

            let def = match lookup_definition(op as u8) {
                Ok(def) => def,
                Err(e) => panic!("definition not found: {e}"),
            };

            let (operands_read, n) = read_operands(&def, &instruction[1..]);
            assert_eq!(n, bytes_read);

            for (i, want) in operands.iter().enumerate() {
                assert_eq!(operands_read[i], *want);
            }
        }
    }

    #[test]
    fn test_definitions() {
        for (i, def) in DEFINITIONS.iter().enumerate() {
            assert_eq!(Opcode::try_from(i as u8).unwrap().to_string(), def.name);
        }
    }

    #[test]
    fn test_try_from() {
        for (i, opcode) in Opcode::iter().enumerate() {
            assert_eq!(opcode, Opcode::try_from(i as u8).unwrap());
        }
    }
}
