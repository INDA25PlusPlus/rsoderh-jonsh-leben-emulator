use parsable::{CharLiteral, CharRange, OnePlus, Parsable, Span};

use crate::instruction::Data16;

#[derive(Clone, Debug, PartialEq, Eq, Parsable)]
pub struct LiteralNumber {
    digits: OnePlus<HexDigit>,
    base: Option<Base>,
}

fn to_u16(literal: LiteralNumber) -> Option<u16> {
    fn parse_hex_digit(digit: HexDigit) -> Option<u8> {
        Some(match &digit.span[0..] {
            b"0" => 0x0, b"1" => 0x1, b"2" => 0x2, b"3" => 0x3,
            b"4" => 0x4, b"5" => 0x5, b"6" => 0x6, b"7" => 0x7,
            b"8" => 0x8, b"9" => 0x9, b"A" => 0xa, b"B" => 0xb,
            b"C" => 0xc, b"D" => 0xd, b"E" => 0xe, b"F" => 0xf,
            _ => return None,
        })
    }

    let base = match literal.base {
        Some(base) => match base {
            Base::Hex(..) => 16,
            Base::Octal(..) => 8,
        },
        None => 10,
    };

    let mut acc = 0_u32;
    for unparsed_digit in literal.digits.nodes {
        let digit = parse_hex_digit(unparsed_digit)? as u32;
        if digit >= base { return None; }
        acc *= base;
        acc += digit;
    }
    if acc > 0xffff { return None; }
    Some(acc as u16)
}

impl TryFrom<LiteralNumber> for u8 {
    type Error = ();

    fn try_from(value: LiteralNumber) -> Result<Self, Self::Error> {
        to_u16(value).and_then(|v| (v < 0x100).then_some(v as u8)).ok_or(())
    }
}

impl TryFrom<LiteralNumber> for u16 {
    type Error = ();

    fn try_from(value: LiteralNumber) -> Result<Self, Self::Error> {
        to_u16(value).ok_or(())
    }
}

impl TryFrom<LiteralNumber> for Data16 {
    type Error = ();

    fn try_from(value: LiteralNumber) -> Result<Self, Self::Error> {
        u16::try_from(value).map(|v| v.into())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable)]
pub enum Base {
    Hex(CharLiteral<b'H'>),
    Octal(CharLiteral<b'Q'>),
}

pub type HexDigit = Span<HexDigitInner>;

#[derive(Clone, Debug, PartialEq, Eq, Parsable)]
pub enum HexDigitInner {
    Numeral(CharRange<b'0', b'9'>),
    AToF(CharRange<b'A', b'F'>),
}
