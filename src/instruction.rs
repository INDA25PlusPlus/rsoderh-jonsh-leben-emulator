#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum Register8 {
    B, C, D, E, H, L, M, A,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum Register16 {
    Bc, De, Hl, Sp,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum Register {
    Register8(Register8),
    Register16(Register16),
}

pub type Data8 = u8;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct Data16 { low: Data8, high: Data8, }

pub type Address = Data16;

pub type Port = Data8;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum Condition {
    Carry, NoCarry, Zero, NoZero, Positive, Minus, ParityEven, ParityOdd,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum ResetNumber {
    R0, R1, R2, R3, R4, R5, R6, R7,
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