#[macro_export]
macro_rules! pop {
    ($vec:expr) => {{
        $vec.pop().unwrap()
    }};
}
#[macro_export]
macro_rules! errorf {
    ($($arg:tt)*) => {{
        eprintln!($($arg)*);
        std::process::exit(1);
    }};
}
#[macro_export]
macro_rules! Vm {
    ($vec:expr) => { 
        Vm {
            program: $vec,
            ..Default::default()
        }
    };
}



use crate::word::*;
use crate::instruction::*;
#[derive(Default)]
pub struct Vm
{
    pub program : Vec<Instruction>,
    pub stack   : Vec<Word>,
    pub program_counter : usize,
    pub halt    : bool,
    pub zero    : bool,
    pub greater : bool,
    pub eqauls  : bool,
    pub lesser  : bool,
}
pub enum Fault
{
    Ok,
    Overflow,
    Underflow,
    Div_By_Zero,
    Bad_Operand(&'static str),
}
use crate::String;
fn error_info(fault : Fault) -> String
{
    match fault
    {
        Fault::Ok => String!("OK"),
        Fault::Overflow => String!("OVERFLOW"),
        Fault::Underflow => String!("UNDERFLOW"),
        Fault::Div_By_Zero => String!("DIV_BY_ZERO"),
        Fault::Bad_Operand(msg) => format!("BAD_OPERAND  {}",msg),
    }
}
pub fn dump_vm(vm : &Vm)
{
    println!("Stack :");
    for val in vm.stack.iter()
    {
        println!("{}",val);
    }
}
impl Vm
{
    pub fn exec_prog(&mut self)
    {
        while !self.halt
        {
            self.exec_instruction();
        }
    }
    pub fn exec_instruction(&mut self)
    {
        let res = self.__exec_instruction();
        match res
        {
            Fault::Ok => {}
            _   => 
            {
                dump_vm(self);
                errorf!("Error : {}",error_info(res));
            }
        }
    }
    fn __exec_instruction(&mut self) -> Fault
    {
        match &self.program[self.program_counter]
        {
            Instruction::Push{val} =>
            {
                self.stack.push(val.clone());
                self.program_counter += 1;
            }
            Instruction::Plus =>
            {
                if self.stack.len() < 2 {
                    return Fault::Underflow;
                }
                let a = pop!(self.stack);
                let b = pop!(self.stack);
                self.stack.push(a+b);
                self.program_counter += 1;
            }
            Instruction::Minus =>
            {
                if self.stack.len() < 2 {
                    return Fault::Underflow;
                }
                let a = pop!(self.stack);
                let b = pop!(self.stack);
                self.stack.push(a-b);
                self.program_counter += 1;
            }            
            Instruction::Mult =>
            {
                if self.stack.len() < 2 {
                    return Fault::Underflow;
                }
                let a = pop!(self.stack);
                let b = pop!(self.stack);
                self.stack.push(a*b);
                self.program_counter += 1;
            }
            Instruction::Div =>
            {
                if self.stack.len() < 2 {
                    return Fault::Underflow;
                }
                let a = pop!(self.stack);
                let b = pop!(self.stack);
                if let Word::Int(b) = b && b == 0 {
                    return Fault::Div_By_Zero;
                }
                self.stack.push(a/b);
                self.program_counter += 1;
            }
            Instruction::Dup{val} => 
            {
                let idx = *val as usize;
                if *val < 0{
                    return Fault::Underflow;
                }
                if idx >= self.stack.len(){
                    return Fault::Overflow;
                }
                self.stack.push(self.stack[self.stack.len()-1-idx].clone());
                self.program_counter += 1;
            }
            Instruction::Halt => self.halt = true,
            Instruction::Nop => self.program_counter += 1,
            Instruction::Jmp{val} =>
            {
                if *val as usize >= self.program.len(){
                    return Fault::Bad_Operand("jumping to outside the program");
                }
                self.program_counter = *val as usize;
            }
            Instruction::Cmp =>
            {
                if self.stack.len() < 2 {
                    return Fault::Underflow;
                }
                let a = pop!(self.stack);
                let b = pop!(self.stack);
                if a > b{
                    self.greater = true;
                }
                if a == b{
                    self.eqauls = true;
                }
                if a < b{
                    self.lesser = true;
                }
                self.program_counter += 1;
            }
            Instruction::JmpIfZero{val} =>
            {
                if *val as usize >= self.program.len(){
                    return Fault::Bad_Operand("jumping to outside the program limits");
                }
                let a = pop!(self.stack);
                if let Word::Int(a) = a && a == 0{
                    self.program_counter = *val;
                }
                else{
                    self.program_counter += 1;
                }
            }
            Instruction::SetEquals =>
            {
                if self.stack.len() < 2 {
                    return Fault::Underflow;
                }
                let a = pop!(self.stack);
                let b = pop!(self.stack);
                self.stack.push(Word::Int((a==b) as i64));
                self.program_counter += 1;
            }
            Instruction::SetGreater =>
            {
                if self.stack.len() < 2 {
                    return Fault::Underflow;
                }
                let a = pop!(self.stack);
                let b = pop!(self.stack);
                self.stack.push(Word::Int((a>b) as i64));
                self.program_counter += 1;
            }
            Instruction::SetLess =>
            {
                if self.stack.len() < 2 {
                    return Fault::Underflow;
                }
                let a = pop!(self.stack);
                let b = pop!(self.stack);
                self.stack.push(Word::Int((a<b) as i64));
                self.program_counter += 1;
            }
            Instruction::SetZero =>
            {
                if self.stack.len() < 1 {
                    return Fault::Underflow;
                }
                let a = pop!(self.stack);
                self.stack.push(Word::Int((a==Word::Int(0)) as i64));
                self.program_counter += 1;
            }
        }
        return Fault::Ok;
    }
}