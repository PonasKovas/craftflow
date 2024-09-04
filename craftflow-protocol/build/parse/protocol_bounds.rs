use nom::{
	branch::alt,
	bytes::complete::tag,
	character::complete::digit1,
	combinator::{map, map_res},
	sequence::preceded,
	IResult,
};
use std::collections::BTreeMap;

#[derive(Debug)]
pub enum ProtocolBounds {
	/// 123
	Concrete(u32),
	/// 123-134
	Range(u32, u32),
	/// 123+
	From(u32),
	/// 123-
	Until(u32),
}

pub fn calculate_matching_versions(bounds: &[ProtocolBounds]) -> Vec<u32> {
	let mut versions = crate::SUPPORTED_PROTOCOL_VERSIONS
		.clone()
		.into_iter()
		.map(|v| (v, false))
		.collect::<BTreeMap<_, _>>();

	for bound in bounds {
		match bound {
			&ProtocolBounds::From(start) => {
				versions.iter_mut().for_each(|(v, enabled)| {
					*enabled |= *v >= start;
				});
			}
			&ProtocolBounds::Until(end) => {
				versions.iter_mut().for_each(|(v, enabled)| {
					*enabled |= *v <= end;
				});
			}
			&ProtocolBounds::Range(start, end) => {
				versions.iter_mut().for_each(|(v, enabled)| {
					*enabled |= (*v >= start) && (*v <= end);
				});
			}
			&ProtocolBounds::Concrete(specific) => {
				versions.iter_mut().for_each(|(v, enabled)| {
					*enabled |= *v == specific;
				});
			}
		}
	}

	versions
		.into_iter()
		.filter_map(|(v, enabled)| if enabled { Some(v) } else { None })
		.collect()
}

pub fn parse_protocol_bounds(input: &str) -> IResult<&str, ProtocolBounds> {
	let (input, first) = parse_u32(input)?;

	alt((
		map(preceded(tag("-"), parse_u32), move |second| {
			ProtocolBounds::Range(first, second)
		}),
		map(tag("+"), move |_| ProtocolBounds::From(first)),
		map(tag("-"), move |_| ProtocolBounds::Until(first)),
		move |_| Ok((input, ProtocolBounds::Concrete(first))),
	))(input)
}

fn parse_u32(input: &str) -> IResult<&str, u32> {
	map_res(digit1, |number| u32::from_str_radix(number, 10))(input)
}
