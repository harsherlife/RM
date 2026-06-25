#[macro_export]
macro_rules! create_file {
    ($name:expr) => {{
        std::fs::File::create($name).expect("Failed to create file")
    }};
}
#[macro_export]
macro_rules! open_file {
    ($name:expr) => {
        std::fs::File::open($name).expect("Failed to open file")
    }
}
#[macro_export]
macro_rules! file_len {
    ($file:expr) => {
       $file.metadata().expect("Failed to get metadata").len() 
    }
}

#[macro_export]
macro_rules! write_bytes_to_file {
    ($file:expr, $contents:expr) => {{
        use std::io::Write;
        $file.write_all($contents).expect("Failed to write to file");
    }};
}
#[macro_export]
macro_rules! read_bytes {
    ($name:expr) => {
        std::fs::read($name).unwrap()
    }
}
#[macro_export]
macro_rules! read_file {
    ($path:expr) => {{
        let mut val = std::fs::read_to_string($path).expect("Unable to open file");
        val.push('\n');
        val
    }};
}
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
macro_rules! size_of {
    ($type:ty) => {
        std::mem::size_of::<$type>()
    };
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


pub fn write_prog_to_file(prog: Vec<Instruction>,file_name: &str)
{
    let byte_slice  = unsafe {
        std::slice::from_raw_parts(
            prog.as_ptr() as *const u8,
            prog.len() * size_of!(Instruction),
        )
    };
    let mut file = create_file!(file_name);
    write_bytes_to_file!(file,byte_slice);
}

pub fn read_prog_from_file(file_name: &str) -> Vec<Instruction> 
{
    let mut file = open_file!(file_name);
    let file_size = file_len!(file) as usize;
    let instr_size = size_of!(Instruction);

    assert_eq!(file_size % instr_size, 0);
    let num_instrs = file_size / instr_size;

    let mut vec = Vec::with_capacity(num_instrs);
    for instr in &mut vec 
    {
        *instr = Instruction::Nop;
    }
    let byte_slice = unsafe {
        std::slice::from_raw_parts_mut(
            vec.as_mut_ptr() as *mut u8,
            file_size,
        )
    };
    use std::io::Read;
    file.read_exact(byte_slice).expect("Failed to read all bytes");
    return vec;
}



#[repr(C)]
// #[derive(Copy,Clone)]
pub enum Instruction
{
    Nop,
    Push{val:Word},
    Dup{val:i64},
    Plus,
    Minus,
    Mult,
    Div,
    Jmp{val:usize},
    Cmp,
    SetEquals,
    SetGreater,
    SetLess,
    SetZero,
    JmpIfZero{val:usize},
    Halt,
}

pub enum Fault
{
    Ok,
    Overflow,
    Underflow,
    Div_By_Zero,
    Bad_Operand(&'static str),
}
use crate::word::*;
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