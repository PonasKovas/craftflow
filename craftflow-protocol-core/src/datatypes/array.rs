use crate::{Error, MCPRead, MCPWrite, Result};
use shallowclone::ShallowClone;
use std::{borrow::Cow, fmt::Debug, io::Write, marker::PhantomData};

#[derive(ShallowClone, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Array<'a, #[shallowclone(skip)] LEN, #[shallowclone(skip)] T: Clone> {
	pub data: Cow<'a, [T]>,
	_phantom: PhantomData<LEN>,
}

impl<'a, LEN, T: Clone> Array<'a, LEN, T> {
	pub fn new(data: impl Into<Cow<'a, [T]>>) -> Self {
		Self {
			data: data.into(),
			_phantom: PhantomData,
		}
	}
}

impl<'a, LEN: MCPRead<'a>, T: MCPRead<'a> + Clone> MCPRead<'a> for Array<'a, LEN, T>
where
	LEN: TryInto<usize> + Debug + Copy,
	// copy isnt really required but just makes stuff easier here,
	// and I assume there wont be a type that isnt Copy
{
	fn read(input: &'a [u8]) -> Result<(&'a [u8], Self)> {
		let mut data = Vec::new();

		let (mut input, len) = LEN::read(input)?;
		let len: usize = len
			.try_into()
			.map_err(|_| Error::InvalidData(format!("{len:?} could not be converted to usize")))?;

		for _ in 0..len {
			match T::read(input) {
				Ok((i, element)) => {
					input = i;
					data.push(element);
				}
				Err(e) => return Err(e),
			}
		}

		Ok((input, Self::new(data)))
	}
}

impl<'a, LEN: MCPWrite, T: MCPWrite + Clone> MCPWrite for Array<'a, LEN, T>
where
	usize: TryInto<LEN>,
{
	fn write(&self, output: &mut impl Write) -> Result<usize> {
		let mut written = 0;

		let len: LEN = self.data.len().try_into().map_err(|_| {
			Error::InvalidData(format!(
				"Could not convert {} to {}",
				self.data.len(),
				std::any::type_name::<LEN>()
			))
		})?;

		written += len.write(output)?;

		for element in self.data.iter() {
			written += element.write(output)?;
		}

		Ok(written)
	}
}
