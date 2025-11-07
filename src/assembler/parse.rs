pub mod instruction;
mod literals;
mod token;

use std::fmt::Debug;
use parsable::{CharLiteral, CharRange, EndOfStream, Ignore, Parsable, WithIndex, ZeroPlus, ok_or_throw};

use crate::assembler::{labels::Label, parse::{instruction::Statement, literals::LiteralNumber, token::{Colon, EndOfAssembly, Origin, Semicolon}}};

#[derive(Clone, PartialEq, Eq, Parsable)]
pub struct SourceFile {
    _0: WsNl,
    _1: ZeroPlus<CommentOnlyLine>,
    pub origin_line: Option<OriginLine>,
    pub lines: ZeroPlus<StatementLine>,
    end: EndOfAssemblyLine,
    _2: ZeroPlus<CommentOnlyLine>,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable)]
pub struct CommentOnlyLine(CommentSegment, WsNl);

#[derive(Clone, PartialEq, Eq)]
pub struct OriginLine {
    pub label: Option<LabelSegment>,
    keyword: Origin,
    _0: Ws,
    pub address: WithIndex<LiteralNumber>,
    _1: WsNl,
}

impl<'a> Parsable<'a> for OriginLine {
    fn parse(stream: &mut parsable::ScopedStream<'a>) -> parsable::ParseOutcome<Self>
    where
        Self: Sized
    {
        stream.scope(|stream| {
            Some(Ok(OriginLine {
                label: ok_or_throw!(Option::<LabelSegment>::parse(stream)?),
                keyword: ok_or_throw!(Origin::parse(stream)?),
                _0: ok_or_throw!(Ws::parse_or_error(stream)),
                address: ok_or_throw!(WithIndex::<LiteralNumber>::parse_or_error(stream)),
                _1: ok_or_throw!(WsNl::parse_or_error(stream)),
            }))
        })
    }

    fn error() -> parsable::ParseError {
        String::from("OriginLine")
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable)]
pub struct StatementLine {
    pub content: StatementLineContent,
    _0: WsNl,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable)]
pub enum StatementLineContent {
    Labeled(LabelSegment, Option<StatementSegment>, Option<CommentSegment>),
    NoLabel(StatementSegment, Option<CommentSegment>),
    OnlyComment(CommentSegment),
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable)]
pub struct EndOfAssemblyLine(Option<LabelSegment>, EndOfAssembly, WsNl, EndOfStream);

#[derive(Clone, PartialEq, Eq)]
pub struct LabelSegment(pub WithIndex<Label>, Colon, Ws);

impl<'a> Parsable<'a> for LabelSegment {
    fn parse(stream: &mut parsable::ScopedStream<'a>) -> parsable::ParseOutcome<Self>
    where
        Self: Sized
    {
        stream.scope(|stream| {
            Some(Ok(LabelSegment(
                ok_or_throw!(WithIndex::<Label>::parse(stream)?),
                ok_or_throw!(Colon::parse(stream)?),
                ok_or_throw!(Ws::parse(stream)?),
            )))
        })
    }

    fn error() -> parsable::ParseError {
        todo!()
    }
}

impl Debug for LabelSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("LabelSegment").field(&self.0.node).field(&self.1).field(&self.2).finish()
    }
}

#[derive(Clone, PartialEq, Eq, Parsable)]
pub struct StatementSegment {
    pub statement: WithIndex<Statement>,
    _0: Ws,
}

impl Debug for StatementSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StatementSegment").field("statement", &self.statement.node).field("_0", &self._0).finish()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable)]
pub struct CommentSegment(Semicolon, ZeroPlus<NonNlChar>);

pub type WsNl = Ignore<ZeroPlus<WsNlChar>>;

#[derive(Clone, Debug, PartialEq, Eq, Parsable)]
pub enum WsNlChar {
    Ws(WsChar),
    Nl(Nl),
}

pub type Ws = Ignore<ZeroPlus<WsChar>>;

#[derive(Clone, Debug, PartialEq, Eq, Parsable)]
pub enum WsChar {
    Space(CharLiteral<b' '>),
    Tab(CharLiteral<b'\t'>),
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable)]
pub enum Nl {
    Crlf(#[literal = b"\r\n"] ()),
    Cr(CharLiteral<b'\r'>),
    Lf(CharLiteral<b'\n'>),
}

pub type NonNlChar = Ignore<NonNlCharInner>;

#[derive(Clone, Debug, PartialEq, Eq, Parsable)]
pub enum NonNlCharInner {
    Tab(CharLiteral<b'\t'>),
    Other(CharRange<b' ', b'~'>),
}

#[cfg(test)]
mod tests {
    use super::*;
    use parsable::*;

    #[test]
    fn option() {
        #[derive(Parsable)]
        struct A {
            x: Option<CharLiteral<b'x'>>,
            y: CharLiteral<b'y'>,
            end: EndOfStream,
        }

        #[derive(Parsable)]
        struct B {
            x: CharLiteral<b'x'>,
            y: Option<CharLiteral<b'y'>>,
            end: EndOfStream,
        }

        let source_1 = b"xy";
        let source_2 = b"x";
        let source_3 = b"y";
        let source_4 = b"yx";

        assert!(matches!(A::parse(&mut ScopedStream::new(source_1)), Some(Ok(..))));
        assert!(matches!(A::parse(&mut ScopedStream::new(source_2)), Some(Err(..))));
        assert!(matches!(A::parse(&mut ScopedStream::new(source_3)), Some(Ok(..))));
        assert!(matches!(A::parse(&mut ScopedStream::new(source_4)), Some(Err(..))));

        assert!(matches!(B::parse(&mut ScopedStream::new(source_1)), Some(Ok(..))));
        assert!(matches!(B::parse(&mut ScopedStream::new(source_2)), Some(Ok(..))));
        assert!(matches!(B::parse(&mut ScopedStream::new(source_3)), None));
        assert!(matches!(B::parse(&mut ScopedStream::new(source_4)), None));
    }
}
