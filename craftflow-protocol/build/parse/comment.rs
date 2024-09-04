use nom::{
	bytes::complete::{tag, take_until},
	IResult,
};

pub fn parse_comment(input: &str) -> IResult<&str, &str> {
	let (input, _) = tag("//")(input)?;
	let (input, comment) = take_until("\n")(input)?;

	Ok((input, comment))
}
