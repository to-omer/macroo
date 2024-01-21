#[derive(Debug, Clone)]
pub struct Ast {
    pub commands: Vec<Command>,
}

#[derive(Debug, Clone)]
pub enum Command {
    Stack(Stack),
    Heap(Heap),
    Arith(Arith),
    Flow(Flow),
    Io(Io),
}

#[derive(Debug, Clone)]
pub struct Number(pub i64);

#[derive(Debug, Clone)]
pub enum Stack {
    Push(Number),
    Duplicate,
    Swap,
    Discard,
    Copy(Number),
    Slide(Number),
}

#[derive(Debug, Clone)]
pub enum Heap {
    Store,
    Retrieve,
}

#[derive(Debug, Clone)]
pub enum Arith {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

#[derive(Debug, Clone)]
pub struct Label(pub Vec<bool>);

#[derive(Debug, Clone)]
pub enum Flow {
    Mark(Label),
    Call(Label),
    Jump(Label),
    JumpIfZero(Label),
    JumpIfNeg(Label),
    Return,
    Exit,
}

#[derive(Debug, Clone)]
pub enum Io {
    OutputChar,
    OutputNum,
    ReadChar,
    ReadNum,
}
