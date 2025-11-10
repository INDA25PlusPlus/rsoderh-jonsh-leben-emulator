use std::ops::Deref;

use parsable::Parsable;

use crate::assembler::labels::{Label, LabelLookup};
use crate::assembler::parse::literals::{LiteralNumber, LiteralString};
use crate::assembler::parse::Ws;
use crate::assembler::parse::token::*;
use crate::instruction::{Address, Condition, Data16, Instruction, Register, RegisterPair, RegisterPairIndirect, RegisterPairOrStatus};

#[derive(Clone, Debug, PartialEq, Eq, Parsable)]
pub enum Statement {
    DataStatement(DataStatement),
    Instruction(ParsedInstruction),
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable)]
pub enum LiteralStringOrNumber {
    String(LiteralString),
    Number(LiteralNumber),
}

impl LiteralStringOrNumber {
    pub fn get(self) -> Option<Box<[u8]>> {
        match self {
            LiteralStringOrNumber::String(literal_string) => {
                Some(literal_string.contents.span.clone().into_boxed_slice())
            },
            LiteralStringOrNumber::Number(literal_number) => {
                let value: u8 = literal_number.try_into().ok()?;
                Some(Box::new([value]))
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable)]
pub enum DataStatement {
    DefineByte(DefineByte, Ws, LiteralStringOrNumber),
    DefineWord(DefineWord, Ws, LabelOrLiteralNumber),
    DefineStorage(DefineStorage, Ws, LiteralNumber),
}

impl DataStatement {
    pub fn byte_length(&self) -> Option<u16> {
        match self {
            DataStatement::DefineByte(_, _, literal) => {
                match literal {
                    LiteralStringOrNumber::String(literal_string) => {
                        Some(literal_string.contents.span.len() as u16)
                    },
                    LiteralStringOrNumber::Number(_) => {
                        Some(1)
                    },
                }
            }
            DataStatement::DefineWord(..) => Some(2),
            DataStatement::DefineStorage(_, _, literal_number) => {
                literal_number.clone().try_into().ok()
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable)]
pub enum LabelOrLiteralNumber {
    Label(Label),
    LiteralNumber(LiteralNumber),
}

impl LabelOrLiteralNumber {
    pub fn get(self, label_lookup: &LabelLookup) -> Option<Address> {
        match self {
            LabelOrLiteralNumber::Label(label) => {
                label_lookup.get(label)
            }
            LabelOrLiteralNumber::LiteralNumber(literal_number) => {
                literal_number.try_into().ok()
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable)]
pub struct ParsedInstruction {
    inner: ParsedInstructionInner,
}

impl ParsedInstruction {
    pub fn into_inner(self, label_lookup: &LabelLookup) -> Option<Instruction> {
        use Instruction as I;
        use ParsedInstructionInner as PI;
        match self.inner {
            PI::Mov(_, _, r1, _, _, _, r2) => Some(I::Mov(r1, r2)),
            PI::Mvi(_, _, r1, _, _, _, data) => Some(I::Mvi(r1, data.try_into().ok()?)),
            PI::Lxi(_, _, rp, _, _, _, data) => Some(I::Lxi(rp, data.get(label_lookup)?.into())),
            PI::Lda(_, _, label) => Some(I::Lda(label_lookup.get(label)?)),
            PI::Sta(_, _, label) => Some(I::Sta(label_lookup.get(label)?)),
            PI::Lhld(_, _, data) => Some(I::Lhld(data.get(label_lookup)?)),
            PI::Shld(_, _, data) => Some(I::Shld(data.get(label_lookup)?)),
            PI::Ldax(_, _, rp) => Some(I::Ldax(rp)),
            PI::Stax(_, _, rp) => Some(I::Stax(rp)),
            PI::Xchg(_) => Some(I::Xchg),

            PI::Add(_, _, r1) => Some(I::Add(r1)),
            PI::Adi(_, _, data) => Some(I::Adi(data.try_into().ok()?)),
            PI::Adc(_, _, r1) => Some(I::Adc(r1)),
            PI::Aci(_, _, data) => Some(I::Aci(data.try_into().ok()?)),
            PI::Sub(_, _, r1) => Some(I::Sub(r1)),
            PI::Sui(_, _, data) => Some(I::Sui(data.try_into().ok()?)),
            PI::Sbb(_, _, r1) => Some(I::Sbb(r1)),
            PI::Sbi(_, _, data) => Some(I::Sbi(data.try_into().ok()?)),
            PI::Inr(_, _, r1) => Some(I::Inr(r1)),
            PI::Dcr(_, _, r1) => Some(I::Dcr(r1)),
            PI::Inx(_, _, rp) => Some(I::Inx(rp)),
            PI::Dcx(_, _, rp) => Some(I::Dcx(rp)),
            PI::Dad(_, _, rp) => Some(I::Dad(rp)),
            PI::Daa(_) => Some(I::Daa),

            PI::Ana(_, _, r1) => Some(I::Ana(r1)),
            PI::Ani(_, _, data) => Some(I::Ani(data.try_into().ok()?)),
            PI::Xra(_, _, r1) => Some(I::Xra(r1)),
            PI::Xri(_, _, data) => Some(I::Xri(data.try_into().ok()?)),
            PI::Ora(_, _, r1) => Some(I::Ora(r1)),
            PI::Ori(_, _, data) => Some(I::Ori(data.try_into().ok()?)),
            PI::Cmp(_, _, r1) => Some(I::Cmp(r1)),
            PI::Cpi(_, _, data) => Some(I::Cpi(data.try_into().ok()?)),
            PI::Rlc(_) => Some(I::Rlc),
            PI::Rrc(_) => Some(I::Rrc),
            PI::Ral(_) => Some(I::Ral),
            PI::Rar(_) => Some(I::Rar),
            PI::Cma(_) => Some(I::Cma),
            PI::Cmc(_) => Some(I::Cmc),
            PI::Stc(_) => Some(I::Stc),

            PI::Jmp(_, _, address) => Some(I::Jmp(address.get(label_lookup)?)),
            PI::Jc(_, _, address) => Some(I::Jcc(Condition::Carry, address.get(label_lookup)?)),
            PI::Jnc(_, _, address) => Some(I::Jcc(Condition::NoCarry, address.get(label_lookup)?)),
            PI::Jz(_, _, address) => Some(I::Jcc(Condition::Zero, address.get(label_lookup)?)),
            PI::Jnz(_, _, address) => Some(I::Jcc(Condition::NoZero, address.get(label_lookup)?)),
            PI::Jp(_, _, address) => Some(I::Jcc(Condition::Positive, address.get(label_lookup)?)),
            PI::Jm(_, _, address) => Some(I::Jcc(Condition::Minus, address.get(label_lookup)?)),
            PI::Jpe(_, _, address) => Some(I::Jcc(Condition::ParityEven, address.get(label_lookup)?)),
            PI::Jpo(_, _, address) => Some(I::Jcc(Condition::ParityOdd, address.get(label_lookup)?)),
            PI::Call(_, _, address) => Some(I::Call(address.get(label_lookup)?)),
            PI::Cc(_, _, address) => Some(I::Ccc(Condition::Carry, address.get(label_lookup)?)),
            PI::Cnc(_, _, address) => Some(I::Ccc(Condition::NoCarry, address.get(label_lookup)?)),
            PI::Cz(_, _, address) => Some(I::Ccc(Condition::Zero, address.get(label_lookup)?)),
            PI::Cnz(_, _, address) => Some(I::Ccc(Condition::NoZero, address.get(label_lookup)?)),
            PI::Cp(_, _, address) => Some(I::Ccc(Condition::Positive, address.get(label_lookup)?)),
            PI::Cm(_, _, address) => Some(I::Ccc(Condition::Minus, address.get(label_lookup)?)),
            PI::Cpe(_, _, address) => Some(I::Ccc(Condition::ParityEven, address.get(label_lookup)?)),
            PI::Cpo(_, _, address) => Some(I::Ccc(Condition::ParityOdd, address.get(label_lookup)?)),
            PI::Ret(_) => Some(I::Ret),
            PI::Rc(_) => Some(I::Rcc(Condition::Carry)),
            PI::Rnc(_) => Some(I::Rcc(Condition::NoCarry)),
            PI::Rz(_) => Some(I::Rcc(Condition::Zero)),
            PI::Rnz(_) => Some(I::Rcc(Condition::NoZero)),
            PI::Rp(_) => Some(I::Rcc(Condition::Positive)),
            PI::Rm(_) => Some(I::Rcc(Condition::Minus)),
            PI::Rpe(_) => Some(I::Rcc(Condition::ParityEven)),
            PI::Rpo(_) => Some(I::Rcc(Condition::ParityOdd)),
            PI::Rst(_, _, data) => Some(I::Rst(data.try_into().ok()?)),
            PI::Pchl(_) => Some(I::Pchl),

            PI::Push(_, _, rp) => Some(I::Push(rp)),
            PI::Pop(_, _, rp) => Some(I::Pop(rp)),
            PI::Xthl(_) => Some(I::Xthl),
            PI::Sphl(_) => Some(I::Sphl),
            PI::Out(_, _, data) => Some(I::Out(data.try_into().ok()?)),
            PI::In(_, _, data) => Some(I::In(data.try_into().ok()?)),
            PI::Ei(_) => Some(I::Ei),
            PI::Di(_) => Some(I::Di),
            PI::Hlt(_) => Some(I::Hlt),
            PI::Nop(_) => Some(I::Nop),
        }
    }

    pub fn instruction_length(&self) -> u16 {
        match self.inner {
            ParsedInstructionInner::Mov(..) => 1,
            ParsedInstructionInner::Mvi(..) => 2,
            ParsedInstructionInner::Lxi(..) => 3,
            ParsedInstructionInner::Lda(..) => 3,
            ParsedInstructionInner::Sta(..) => 3,
            ParsedInstructionInner::Lhld(..) => 3,
            ParsedInstructionInner::Shld(..) => 3,
            ParsedInstructionInner::Ldax(..) => 1,
            ParsedInstructionInner::Stax(..) => 1,
            ParsedInstructionInner::Xchg(..) => 1,
            ParsedInstructionInner::Add(..) => 1,
            ParsedInstructionInner::Adi(..) => 2,
            ParsedInstructionInner::Adc(..) => 1,
            ParsedInstructionInner::Aci(..) => 2,
            ParsedInstructionInner::Sub(..) => 1,
            ParsedInstructionInner::Sui(..) => 2,
            ParsedInstructionInner::Sbb(..) => 1,
            ParsedInstructionInner::Sbi(..) => 2,
            ParsedInstructionInner::Inr(..) => 1,
            ParsedInstructionInner::Dcr(..) => 1,
            ParsedInstructionInner::Inx(..) => 1,
            ParsedInstructionInner::Dcx(..) => 1,
            ParsedInstructionInner::Dad(..) => 1,
            ParsedInstructionInner::Daa(..) => 1,
            ParsedInstructionInner::Ana(..) => 1,
            ParsedInstructionInner::Ani(..) => 2,
            ParsedInstructionInner::Xra(..) => 1,
            ParsedInstructionInner::Xri(..) => 2,
            ParsedInstructionInner::Ora(..) => 1,
            ParsedInstructionInner::Ori(..) => 2,
            ParsedInstructionInner::Cmp(..) => 1,
            ParsedInstructionInner::Cpi(..) => 2,
            ParsedInstructionInner::Rlc(..) => 1,
            ParsedInstructionInner::Rrc(..) => 1,
            ParsedInstructionInner::Ral(..) => 1,
            ParsedInstructionInner::Rar(..) => 1,
            ParsedInstructionInner::Cma(..) => 1,
            ParsedInstructionInner::Cmc(..) => 1,
            ParsedInstructionInner::Stc(..) => 1,
            ParsedInstructionInner::Jmp(..) => 3,
            ParsedInstructionInner::Jc(..) => 3,
            ParsedInstructionInner::Jnc(..) => 3,
            ParsedInstructionInner::Jz(..) => 3,
            ParsedInstructionInner::Jnz(..) => 3,
            ParsedInstructionInner::Jp(..) => 3,
            ParsedInstructionInner::Jm(..) => 3,
            ParsedInstructionInner::Jpe(..) => 3,
            ParsedInstructionInner::Jpo(..) => 3,
            ParsedInstructionInner::Call(..) => 3,
            ParsedInstructionInner::Cc(..) => 3,
            ParsedInstructionInner::Cnc(..) => 3,
            ParsedInstructionInner::Cz(..) => 3,
            ParsedInstructionInner::Cnz(..) => 3,
            ParsedInstructionInner::Cp(..) => 3,
            ParsedInstructionInner::Cm(..) => 3,
            ParsedInstructionInner::Cpe(..) => 3,
            ParsedInstructionInner::Cpo(..) => 3,
            ParsedInstructionInner::Ret(..) => 1,
            ParsedInstructionInner::Rc(..) => 1,
            ParsedInstructionInner::Rnc(..) => 1,
            ParsedInstructionInner::Rz(..) => 1,
            ParsedInstructionInner::Rnz(..) => 1,
            ParsedInstructionInner::Rp(..) => 1,
            ParsedInstructionInner::Rm(..) => 1,
            ParsedInstructionInner::Rpe(..) => 1,
            ParsedInstructionInner::Rpo(..) => 1,
            ParsedInstructionInner::Rst(..) => 1,
            ParsedInstructionInner::Pchl(..) => 1,
            ParsedInstructionInner::Push(..) => 1,
            ParsedInstructionInner::Pop(..) => 1,
            ParsedInstructionInner::Xthl(..) => 1,
            ParsedInstructionInner::Sphl(..) => 1,
            ParsedInstructionInner::In(..) => 2,
            ParsedInstructionInner::Out(..) => 2,
            ParsedInstructionInner::Ei(..) => 1,
            ParsedInstructionInner::Di(..) => 1,
            ParsedInstructionInner::Hlt(..) => 1,
            ParsedInstructionInner::Nop(..) => 1,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable)]
enum ParsedInstructionInner {
    Mov(Mov, Ws, Register, Ws, Comma, Ws, Register),
    Mvi(Mvi, Ws, Register, Ws, Comma, Ws, LiteralNumber),
    Lxi(Lxi, Ws, RegisterPair, Ws, Comma, Ws, LabelOrLiteralNumber),
    Lda(Lda, Ws, Label),
    Sta(Sta, Ws, Label),
    Lhld(Lhld, Ws, LabelOrLiteralNumber),
    Shld(Shld, Ws, LabelOrLiteralNumber),
    Ldax(Ldax, Ws, RegisterPairIndirect),
    Stax(Stax, Ws, RegisterPairIndirect),
    Xchg(Xchg),

    Add(Add, Ws, Register),
    Adi(Adi, Ws, LiteralNumber),
    Adc(Adc, Ws, Register),
    Aci(Aci, Ws, LiteralNumber),
    Sub(Sub, Ws, Register),
    Sui(Sui, Ws, LiteralNumber),
    Sbb(Sbb, Ws, Register),
    Sbi(Sbi, Ws, LiteralNumber),
    Inr(Inr, Ws, Register),
    Dcr(Dcr, Ws, Register),
    Inx(Inx, Ws, RegisterPair),
    Dcx(Dcx, Ws, RegisterPair),
    Dad(Dad, Ws, RegisterPair),
    Daa(Daa),

    Ana(Ana, Ws, Register),
    Ani(Ani, Ws, LiteralNumber),
    Xra(Xra, Ws, Register),
    Xri(Xri, Ws, LiteralNumber),
    Ora(Ora, Ws, Register),
    Ori(Ori, Ws, LiteralNumber),
    Cmp(Cmp, Ws, Register),
    Cpi(Cpi, Ws, LiteralNumber),
    Rlc(Rlc),
    Rrc(Rrc),
    Ral(Ral),
    Rar(Rar),
    Cma(Cma),
    Cmc(Cmc),
    Stc(Stc),

    Jmp(Jmp, Ws, LabelOrLiteralNumber),
    Jc(Jc, Ws, LabelOrLiteralNumber),
    Jnc(Jnc, Ws, LabelOrLiteralNumber),
    Jz(Jz, Ws, LabelOrLiteralNumber),
    Jnz(Jnz, Ws, LabelOrLiteralNumber),
    Jp(Jp, Ws, LabelOrLiteralNumber),
    Jm(Jm, Ws, LabelOrLiteralNumber),
    Jpe(Jpe, Ws, LabelOrLiteralNumber),
    Jpo(Jpo, Ws, LabelOrLiteralNumber),
    Call(Call, Ws, LabelOrLiteralNumber),
    Cc(Cc, Ws, LabelOrLiteralNumber),
    Cnc(Cnc, Ws, LabelOrLiteralNumber),
    Cz(Cz, Ws, LabelOrLiteralNumber),
    Cnz(Cnz, Ws, LabelOrLiteralNumber),
    Cp(Cp, Ws, LabelOrLiteralNumber),
    Cm(Cm, Ws, LabelOrLiteralNumber),
    Cpe(Cpe, Ws, LabelOrLiteralNumber),
    Cpo(Cpo, Ws, LabelOrLiteralNumber),
    Ret(Ret),
    Rc(Rc),
    Rnc(Rnc),
    Rz(Rz),
    Rnz(Rnz),
    Rp(Rp),
    Rm(Rm),
    Rpe(Rpe),
    Rpo(Rpo),
    Rst(Rst, Ws, LiteralNumber),
    Pchl(Pchl),

    Push(Push, Ws, RegisterPairOrStatus),
    Pop(Pop, Ws, RegisterPairOrStatus),
    Xthl(Xthl),
    Sphl(Sphl),
    In(In, Ws, LiteralNumber),
    Out(Out, Ws, LiteralNumber),
    Ei(Ei),
    Di(Di),
    Hlt(Hlt),
    Nop(Nop),
}
