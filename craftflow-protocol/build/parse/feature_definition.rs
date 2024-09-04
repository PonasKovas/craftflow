use super::{
	comment::parse_comment,
	protocol_bounds::{calculate_matching_versions, parse_protocol_bounds},
	skip::skip,
};
use nom::sequence::{preceded, terminated};
use nom::{
	branch::alt,
	bytes::complete::{tag, take_until},
	character::complete::space1,
	combinator::{cut, eof},
	multi::{many0, separated_list1},
	sequence::delimited,
	IResult,
};

#[derive(Debug, Clone)]
pub struct Feature {
	pub name: String,
	pub enabled_for: Vec<u32>,
}

pub fn parse_features(input: &str) -> IResult<&str, Vec<Feature>> {
	let (input, _) = skip(input)?;
	let (input, features) = many0(terminated(parse_feature, skip))(input)?;

	let (input, _) = eof(input)?;

	Ok((input, features))
}

fn parse_feature(input: &str) -> IResult<&str, Feature> {
	let (input, _) = tag("feature ")(input)?;
	let (input, name) = take_until(" ")(input)?;
	let (input, _) = cut(tag(" @"))(input)?;
	let (input, bounds) = cut(delimited(
		tag("["),
		separated_list1(tag(","), parse_protocol_bounds),
		tag("]"),
	))(input)?;

	Ok((
		input,
		Feature {
			name: name.to_string(),
			enabled_for: calculate_matching_versions(&bounds),
		},
	))
}
