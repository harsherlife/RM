use crate::errorf;
use std::cmp::Ordering;

#[derive(Copy,Clone)]
pub enum Word
{
    Int(i64),
    Uint(u64),
    Ptr(u64),
    Float(f64),
}
impl std::fmt::Display for Word
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        match self 
        {
            Word::Int(int_val)      =>  write!(f,"Int : {}",int_val),
            Word::Uint(uint_val)    =>  write!(f,"Uint : {}",uint_val),
            Word::Ptr(ptr_val)      =>  write!(f,"Ptr : {}",ptr_val),
            Word::Float(float_val)  =>  write!(f,"Float : {}",float_val),
        }
    }
}

fn type_name(word : Word) -> &'static str
{
    match word
    {
        Word::Int(_) => "Int",
        Word::Uint(_) => "Uint",
        Word::Float(_) => "Float",
        Word::Ptr(_) => "Ptr",
    }
}

impl std::ops::Add for Word
{
    type Output = Word;
    fn add(self,rhs: Word) -> Word
    {
        match (self,rhs)
        {
            (Word::Int(a),Word::Int(b))     => Word::Int(a+b),
            (Word::Uint(a),Word::Uint(b))   => Word::Uint(a+b),
            (Word::Float(a),Word::Float(b)) => Word::Float(a+b),
            (Word::Ptr(a),Word::Ptr(b))     => Word::Ptr(a+b),
            _ => errorf!("Mismatched types for addition,expected {} but got {}",type_name(self),type_name(rhs)),
        }
    }
}
impl std::ops::Sub for Word
{
    type Output = Word;
    fn sub(self,rhs: Word) -> Word
    {
        match (self,rhs)
        {
            (Word::Int(a),Word::Int(b))     => Word::Int(a-b),
            (Word::Uint(a),Word::Uint(b))   => Word::Uint(a-b),
            (Word::Float(a),Word::Float(b)) => Word::Float(a-b),
            (Word::Ptr(a),Word::Ptr(b))     => Word::Ptr(a-b),
            _ => errorf!("Mismatched types for subtraction,expected {} but got {}",type_name(self),type_name(rhs)),
        }
    }
}
impl std::ops::Mul for Word
{
    type Output = Word;
    fn mul(self,rhs: Word) -> Word
    {
        match (self,rhs)
        {
            (Word::Int(a),Word::Int(b))     => Word::Int(a*b),
            (Word::Uint(a),Word::Uint(b))   => Word::Uint(a*b),
            (Word::Float(a),Word::Float(b)) => Word::Float(a*b),
            (Word::Ptr(a),Word::Ptr(b))     => Word::Ptr(a*b),
            _ => errorf!("Mismatched types for multiplitcation,expected {} but got {}",type_name(self),type_name(rhs)),
        }
    }
}
impl std::ops::Div for Word
{
    type Output = Word;
    fn div(self,rhs: Word) -> Word
    {
        match (self,rhs)
        {
            (Word::Int(a),Word::Int(b))     => Word::Int(a/b),
            (Word::Uint(a),Word::Uint(b))   => Word::Uint(a/b),
            (Word::Float(a),Word::Float(b)) => Word::Float(a/b),
            (Word::Ptr(a),Word::Ptr(b))     => Word::Ptr(a/b),
            _ => errorf!("Mismatched types for division,expected {} but got {}",type_name(self),type_name(rhs)),
        }
    }
}
impl PartialEq for Word
{
    fn eq(&self,rhs:&Word) -> bool
    {
        match (self,rhs)
        {
            (Word::Int(a),Word::Int(b))     => a == b,
            (Word::Uint(a),Word::Uint(b))   => a == b,
            (Word::Float(a),Word::Float(b)) => a.total_cmp(&b) == Ordering::Equal,
            (Word::Ptr(a),Word::Ptr(b))     => a == b,
            _ => errorf!("Mismatched types for division,expected {} but got {}",type_name(*self),type_name(*rhs)),
        }   
    }
}

impl Eq for Word{}

impl PartialOrd for Word
{
    fn partial_cmp(&self,rhs:&Word) -> Option<Ordering>
    {
        Some(self.cmp(rhs))
    }
}

impl Ord for Word
{
    fn cmp(&self,rhs:&Word) -> Ordering
    {
        match (self,rhs)
        {
            (Word::Int(a),Word::Int(b))     => a.cmp(&b),
            (Word::Uint(a),Word::Uint(b))   => a.cmp(&b),
            (Word::Float(a),Word::Float(b)) => a.total_cmp(&b),
            (Word::Ptr(a),Word::Ptr(b))     => a.cmp(&b),
            _ => errorf!("Mismatched types for division,expected {} but got {}",type_name(*self),type_name(*rhs)),
        }
    }
}