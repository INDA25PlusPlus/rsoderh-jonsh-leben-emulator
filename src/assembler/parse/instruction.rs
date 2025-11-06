use parsable::Parsable;

use crate::assembler::labels::{Label, LabelLookup};
use crate::assembler::parse::number::LiteralNumber;
use crate::assembler::parse::Ws;
use crate::assembler::parse::token::*;
use crate::instruction::{Instruction, Register, RegisterPair};

#[derive(Clone, Debug, PartialEq, Eq, Parsable)]
pub struct ParsedInstruction {
    inner: ParsedInstructionInner,
}

impl ParsedInstruction {
    pub fn get(&self, label_lookup: LabelLookup) -> Option<Instruction> {
        use Instruction as I;
        use ParsedInstructionInner as PII;
        match self.inner.clone() {
            PII::Mov(_, _, from, _, _, _, to) => Some(I::Mov(from, to)),
            PII::Mvi(_, _, to, _, data) => Some(I::Mvi(to, data.try_into().ok()?)),
            PII::Lxi(_, _, to, _, data) => Some(I::Lxi(to, data.try_into().ok()?)),
            PII::Lda(_, _, label) => Some(I::Lda(label_lookup.lookup(label)?)),
            PII::Sta(_, _, label) => Some(I::Sta(label_lookup.lookup(label)?)),
            PII::Lhld(_, _, label) => Some(I::Lhld(label_lookup.lookup(label)?)),
            _ => todo!(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable)]
enum ParsedInstructionInner {
    Mov(Mov, Ws, Register, Ws, Comma, Ws, Register),
    Mvi(Mvi, Ws, Register, Ws, LiteralNumber),
    Lxi(Lxi, Ws, RegisterPair, Ws, LiteralNumber),
    Lda(Lda, Ws, Label),
    Sta(Sta, Ws, Label),
    Lhld(Lhld, Ws, Label),
    // ...
}
