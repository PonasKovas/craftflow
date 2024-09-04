use super::{
	comment::parse_comment,
	protocol_bounds::{calculate_matching_versions, parse_protocol_bounds},
	skip::skip,
};
use nom::{
	branch::alt,
	bytes::complete::{tag, take_until},
	character::complete::space1,
	combinator::{cut, eof},
	multi::{many0, separated_list1},
	sequence::delimited,
	IResult,
};
use nom::{
	combinator::opt,
	sequence::{preceded, terminated},
};

#[derive(Debug, Clone)]
pub struct State {
	pub name: String,
	pub feature_gate: Option<String>,
}

pub fn parse_states(input: &str) -> IResult<&str, Vec<State>> {
	let (input, _) = skip(input)?;
	let (input, features) = many0(terminated(parse_state, skip))(input)?;

	let (input, _) = eof(input)?;

	Ok((input, features))
}

fn parse_state(input: &str) -> IResult<&str, State> {
	let (input, feature_gate) =
		opt(delimited(tag("#[feature("), take_until(")]"), tag(")]\n")))(input)?;
	let (input, _) = tag("state ")(input)?;
	let (input, name) = take_until("\n")(input)?;

	Ok((
		input,
		State {
			name: name.to_string(),
			feature_gate: feature_gate.map(|s| s.to_string()),
		},
	))
}
