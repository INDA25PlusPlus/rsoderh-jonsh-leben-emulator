use std::{fmt::Display, ops::Add};

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum Register8 {
    B,
    C,
    D,
    E,
    H,
    L,
    M,
    A,
}

impl Display for Register8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Register8::B => "B",
            Register8::C => "C",
            Register8::D => "D",
            Register8::E => "E",
            Register8::H => "H",
            Register8::L => "L",
            Register8::M => "M",
            Register8::A => "A",
        })
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum Register16 {
    Bc,
    De,
    Hl,
    Sp,
}

impl Display for Register16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Register16::Bc => "BC",
            Register16::De => "DE",
            Register16::Hl => "HL",
            Register16::Sp => "SP",
        })
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum Register {
    Register8(Register8),
    Register16(Register16),
}

impl Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Register::Register8(register) => register.fmt(f),
            Register::Register16(register) => register.fmt(f),
        }
    }
}

pub type Data8 = u8;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct Data16 {
    low: Data8,
    high: Data8,
}

impl Data16 {
    pub const ZERO: Self = Data16 { low: 0, high: 0 };

    pub fn new(low: Data8, high: Data8) -> Self {
        Self { low, high }
    }

    pub fn value(&self) -> u16 {
        self.low as u16 + (self.high as u16) << 8
    }
}

impl From<u16> for Data16 {
    fn from(value: u16) -> Self {
        Self {
            low: value as u8,
            high: (value >> 8) as u8,
        }
    }
}

impl Add for Data16 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        (self.value() + rhs.value()).into()
    }
}

pub type Address = Data16;

pub type Port = Data8;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum Condition {
    Carry,
    NoCarry,
    Zero,
    NoZero,
    Positive,
    Minus,
    ParityEven,
    ParityOdd,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum ResetNumber {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum Instruction {
    Nop,
    Lxi(Register16, Data16),
    Stax(Register16),
    Inx(Register16),
    Inr(Register8),
    Dcr(Register8),
    Mvi(Register8, Data8),
    Dad(Register16),
    Ldax(Register16),
    Dcx(Register16),
    Rlc,
    Rrc,
    Ral,
    Rar,
    Shld(Address),
    Daa,
    Lhld(Address),
    Cma,
    Sta(Address),
    Stc,
    Lda(Address),
    Cmc,
    Mov(Register8, Register8),
    Hlt,
    Add(Register8),
    Adc(Register8),
    Sub(Register8),
    Sbb(Register8),
    Ana(Register8),
    Xra(Register8),
    Ora(Register8),
    Cmp(Register8),
    Rcc(Condition),
    Pop(Register16),
    Jcc(Condition, Address),
    Jmp(Address),
    Ccc(Address),
    Push(Register8),
    Adi(Data8),
    Aci(Data8),
    Sui(Data8),
    Sbi(Data8),
    Ani(Data8),
    Xri(Data8),
    Ori(Data8),
    Cpi(Data8),
    Rst(ResetNumber),
    Ret,
    Call(Address),
    Out(Port),
    In(Port),
    Xthl,
    Pchl,
    Xchg,
    Di,
    Sphl,
    Ei,
}
