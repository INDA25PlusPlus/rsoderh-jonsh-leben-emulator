use parsable::{CharLiteral, CharRange, Parsable, Span, ZeroPlus};

use crate::instruction::Address;

pub struct LabelLookup {
    
}

impl LabelLookup {
    pub fn lookup(&self, label: Label) -> Option<Address> {
        todo!()
    }
}

pub type Label = Span<LabelInner>;

#[derive(Clone, Debug, PartialEq, Eq, Parsable)]
pub struct LabelInner {
    initial: InitialLabelChar,
    rest: ZeroPlus<LabelChar>,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable)]
enum InitialLabelChar {
    At(CharLiteral<b'@'>),
    QuestionMark(CharLiteral<b'?'>),
    Alpha(CharRange<b'A', b'Z'>),
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable)]
enum LabelChar {
    Alpha(CharRange<b'A', b'Z'>),
    Numerical(CharRange<b'0', b'9'>),
}