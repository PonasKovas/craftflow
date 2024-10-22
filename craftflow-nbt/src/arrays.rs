//! Provides a choice to serialize sequences as `ByteArray`, `IntArray` or `LongArray`.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub(crate) const MAGIC: &str = "_nbt_array_type";
pub(crate) const MAGIC_BYTE_ARRAY: &str = "_nbt_byte_array";
pub(crate) const MAGIC_INT_ARRAY: &str = "_nbt_int_array";
pub(crate) const MAGIC_LONG_ARRAY: &str = "_nbt_long_array";

macro_rules! impl_array_type {
	($(#[$meta:meta])* $mod_name:ident, $struct_name:ident, $magic:ident) => {
	    $(#[$meta])*
		#[repr(transparent)]
		#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
		pub struct $struct_name<T>(pub T);

		$(#[$meta])*
		///
		/// Use this with `#[serde(with = "..."]`
		pub mod $mod_name {
		    use serde::{Serialize, Serializer, Deserialize, Deserializer, de::{VariantAccess, EnumAccess, Visitor}};
			use std::marker::PhantomData;

            pub fn serialize<T: Serialize, S: Serializer>(
                value: &T,
                serializer: S,
            ) -> Result<S::Ok, S::Error> {
                // use a newtype struct with a magic name to signal the serializer for special behaviour
                serializer.serialize_newtype_struct(super::$magic, value)
            }
            pub fn deserialize<'de, T: Deserialize<'de>, D: Deserializer<'de>>(
               	deserializer: D,
            ) -> Result<T, D::Error> {
                // Tell the deserializer that we are expecting an enum variant with a magic name

               	struct V<T>(PhantomData<T>);
               	impl<'de, T: Deserialize<'de>> Visitor<'de> for V<T> {
              		type Value = T;

              		fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
             			formatter.write_str(stringify!(a $struct_name))
              		}
              		fn visit_enum<A: EnumAccess<'de>>(self, data: A) -> Result<Self::Value, A::Error> {
                        let (name, inner): (&str, _) = data.variant()?;

                        if name != super::$magic {
                            return Err(serde::de::Error::unknown_variant(
                                name,
                                &[super::MAGIC_BYTE_ARRAY, super::MAGIC_INT_ARRAY, super::MAGIC_LONG_ARRAY],
                            ));
                        }

                        inner.newtype_variant()
                    }
               	}
               	deserializer.deserialize_enum(super::MAGIC, &[], V(PhantomData))
            }
        }

        impl<T: Serialize> Serialize for $struct_name<T>
        {
            fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                $mod_name::serialize(&self.0, serializer)
            }
        }
        impl<'de, T: Deserialize<'de>> Deserialize<'de> for $struct_name<T>
        {
            fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                $mod_name::deserialize(deserializer).map($struct_name)
            }
        }
	};
}

impl_array_type! {
	/// Allows to (de)serialize a sequence as a `ByteArray`.
	byte_array, ByteArray, MAGIC_BYTE_ARRAY
}
impl_array_type! {
	/// Allows to (de)serialize a sequence as an `IntArray`.
	int_array, IntArray, MAGIC_INT_ARRAY
}
impl_array_type! {
	/// Allows to (de)serialize a sequence as a `LongArray`.
	long_array, LongArray, MAGIC_LONG_ARRAY
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{tag::Tag, to_writer};
	use serde::Serialize;

	#[test]
	fn test_byte_array() {
		#[derive(Serialize)]
		struct Test {
			#[serde(with = "byte_array")]
			bytes: Vec<u8>,
		}

		const BYTES: [u8; 5] = [1, 2, 3, 4, 5];
		let mut buffer = Vec::new();

		let bytes_written = to_writer(
			&mut buffer,
			&Test {
				bytes: BYTES.to_vec(),
			},
		)
		.unwrap();

		assert_eq!(bytes_written, buffer.len(), "written bytes doesnt match");
		#[rustfmt::skip]
		assert_eq!(
			buffer,
			vec![
				Tag::Compound as u8,
    				Tag::ByteArray as u8,
    				0, 5,
    				b'b', b'y', b't', b'e', b's',
    				0, 0, 0, 5,
    				1,
    				2,
    				3,
    				4,
    				5,
				0
			]
		);
	}

	#[test]
	fn test_int_array() {
		#[derive(Serialize)]
		struct Test {
			bytes: IntArray<Vec<u8>>,
		}

		const BYTES: [u8; 4] = [1, 2, 3, 4];
		let mut buffer = Vec::new();

		let bytes_written = to_writer(
			&mut buffer,
			&Test {
				bytes: IntArray(BYTES.to_vec()),
			},
		)
		.unwrap();

		assert_eq!(bytes_written, buffer.len(), "written bytes doesnt match");
		#[rustfmt::skip]
		assert_eq!(
			buffer,
			vec![
				Tag::Compound as u8,
    				Tag::IntArray as u8,
    				0, 5,
    				b'b', b'y', b't', b'e', b's',
    				0, 0, 0, 4,
    				0, 0, 0, 1,
    				0, 0, 0, 2,
    				0, 0, 0, 3,
    				0, 0, 0, 4,
				0
			]
		);
	}

	#[test]
	fn test_long_array() {
		#[derive(Serialize)]
		struct Test {
			bytes: LongArray<Vec<u8>>,
		}

		const BYTES: [u8; 4] = [1, 2, 3, 4];
		let mut buffer = Vec::new();

		let bytes_written = to_writer(
			&mut buffer,
			&Test {
				bytes: LongArray(BYTES.to_vec()),
			},
		)
		.unwrap();

		assert_eq!(bytes_written, buffer.len(), "written bytes doesnt match");
		#[rustfmt::skip]
		assert_eq!(
			buffer,
			vec![
				Tag::Compound as u8,
    				Tag::LongArray as u8,
    				0, 5,
    				b'b', b'y', b't', b'e', b's',
    				0, 0, 0, 4,
    				0, 0, 0, 0, 0, 0, 0, 1,
    				0, 0, 0, 0, 0, 0, 0, 2,
    				0, 0, 0, 0, 0, 0, 0, 3,
    				0, 0, 0, 0, 0, 0, 0, 4,
				0
			]
		);
	}
}
