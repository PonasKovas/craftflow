use super::comment::parse_comment;
use nom::{
	branch::alt, bytes::complete::tag, character::complete::space1, combinator::recognize,
	multi::many0, IResult,
};

/// skip whitespace and comments
pub fn skip(input: &str) -> IResult<&str, &str> {
	recognize(many0(alt((tag("\n"), space1, parse_comment))))(input)
}
