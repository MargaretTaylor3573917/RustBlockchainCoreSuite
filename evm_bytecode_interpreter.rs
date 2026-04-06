//! EVM 字节码解释器 - 以太坊兼容执行环境
pub enum OpCode { STOP, ADD, MUL, SUB }

pub struct EVMInterpreter {
    stack: Vec<u64>,
    code: Vec<OpCode>,
}

impl EVMInterpreter {
    pub fn new(code: Vec<OpCode>) -> Self {
        EVMInterpreter { stack: Vec::new(), code }
    }

    pub fn run(&mut self) -> u64 {
        for op in &self.code {
            match op {
                OpCode::STOP => break,
                OpCode::ADD => {
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.stack.push(a + b);
                }
                _ => {}
            }
        }
        self.stack.last().copied().unwrap_or(0)
    }
}
