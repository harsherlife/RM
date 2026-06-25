macro_rules! create_file {
    ($name:expr) => {{
        std::fs::File::create($name).expect("Failed to create file")
    }};
}
macro_rules! open_file {
    ($name:expr) => {
        std::fs::File::open($name).expect("Failed to open file")
    }
}
macro_rules! file_len {
    ($file:expr) => {
       $file.metadata().expect("Failed to get metadata").len() 
    }
}

macro_rules! write_bytes_to_file {
    ($file:expr, $contents:expr) => {{
        use std::io::Write;
        $file.write_all($contents).expect("Failed to write to file");
    }};
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
macro_rules! size_of {
    ($type:ty) => {
        std::mem::size_of::<$type>()
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
    for _ in 0..num_instrs 
    {
        vec.push(Instruction::Nop);
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


use crate::word::*;
#[repr(C)]
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