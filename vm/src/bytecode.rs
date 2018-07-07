/*
 * Implement python as a virtual machine with bytecodes.
 */

/*
let load_const_string = 0x16;
let call_function = 0x64;
*/

/*
 * Primitive instruction type, which can be encoded and decoded.
 */
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct CodeObject {
    pub instructions: Vec<Instruction>,
    pub label_map: HashMap<Label, usize>,
}

impl CodeObject {
    pub fn new() -> CodeObject {
        CodeObject {
            instructions: Vec::new(),
            label_map: HashMap::new(),
        }
    }
}

pub type Label = usize;

#[derive(Debug, Clone)]
pub enum Instruction {
    LoadName { name: String },
    StoreName { name: String },
    LoadConst { value: Constant },
    UnaryOperation { op: UnaryOperator },
    BinaryOperation { op: BinaryOperator },
    Pop,
    GetIter,
    Pass,
    Continue,
    Break,
    Jump { target: Label },
    JumpIf { target: Label },
    MakeFunction { code: CodeObject },
    CallFunction { count: usize },
    ForIter,
    ReturnValue,
    PushBlock { start: Label, end: Label },
    PopBlock,
    BuildTuple { size: usize },
    BuildList { size: usize },
    BuildMap { size: usize },
}

#[derive(Debug, Clone)]
pub enum Constant {
    Integer { value: i32 }, // TODO: replace by arbitrary big int math.
    // TODO: Float { value: f64 },
    String { value: String },
    None,
}

#[derive(Debug, Clone)]
pub enum BinaryOperator {
    Power,
    Multiply,
    MatrixMultiply,
    Divide,
    FloorDivide,
    Modulo,
    Add,
    Subtract,
    Lshift,
    Rshift,
    And,
    Xor,
    Or,
}

#[derive(Debug, Clone)]
pub enum UnaryOperator {
    Not,
    Minus,
    Plus,
}

/*
Maintain a stack of blocks on the VM.
pub enum BlockType {
    Loop,
    Except,
}
*/
