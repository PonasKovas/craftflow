use crate::{Error, MCPRead, MCPWrite, Result};
use std::{fmt::Debug, io::Write, marker::PhantomData};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Array<LEN, T> {
	pub data: Vec<T>,
	_phantom: PhantomData<LEN>,
}

impl<LEN, T> Array<LEN, T> {
	pub fn new(data: Vec<T>) -> Self {
		Self {
			data,
			_phantom: PhantomData,
		}
	}
}

impl<LEN: MCPRead, T: MCPRead> MCPRead for Array<LEN, T>
where
	LEN: TryInto<usize> + Debug + Copy,
	// copy just makes stuff easier here, and I assume there wont be a type that isnt Copy
{
	fn read(input: &mut [u8]) -> Result<(&mut [u8], Self)> {
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

impl<LEN: MCPWrite, T: MCPWrite> MCPWrite for Array<LEN, T>
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

		for element in &self.data {
			written += element.write(output)?;
		}

		Ok(written)
	}
}
