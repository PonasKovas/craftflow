use super::util::AsTokenStream;
use proc_macro2::TokenStream;
use quote::quote;
use serde::{de::Visitor, Deserialize};
use std::error::Error;

/// The bounds for protocol versions
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum Bounds {
    /// *
    All,
    /// X+
    From(u32),
    /// X-
    UpTo(u32),
    /// X-Y
    Range(u32, u32),
    /// X
    Concrete(u32),
}

pub fn parse_bounds(input: &str) -> Result<Bounds, Box<dyn Error>> {
    if input == "*" {
        Ok(Bounds::All)
    } else if let Some(stripped) = input.strip_suffix("+") {
        Ok(Bounds::From(stripped.parse()?))
    } else if let Some(stripped) = input.strip_suffix("-") {
        return Ok(Bounds::UpTo(stripped.parse()?))
    } else {
        let parts: Vec<&str> = input.split("-").collect();

        if parts.len() == 2 {
            Ok(Bounds::Range(parts[0].parse()?, parts[1].parse()?))
        } else if parts.len() == 1 {
            Ok(Bounds::Concrete(parts[0].parse()?))
        } else {
            Err("cant be more than one - symbol")?
        }
    }

}

impl<'de> Deserialize<'de> for Bounds {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        deserializer.deserialize_str(BoundsVisitor)
    }
}

struct BoundsVisitor;

impl<'de> Visitor<'de> for BoundsVisitor {
    type Value = Bounds;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a string representing protocol version bounds")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error, {
        parse_bounds(v).map_err(serde::de::Error::custom)
    }
}

pub trait BoundsMethods {
    fn as_match_pattern(&self) -> TokenStream;
    /// Checks if a version matches the bounds
    fn contain(&self, version: u32) -> bool;
}

impl BoundsMethods for  [Bounds] {
    fn as_match_pattern(&self) -> TokenStream {
        let mut items = Vec::new();

        for bound in self {
            items.push(match bound {
                Bounds::All => format!("_"),
                Bounds::From(from) => format!("{from}.."),
                Bounds::UpTo(to) => format!("..={to}"),
                Bounds::Range(from, to) => format!("{from}..={to}"),
                Bounds::Concrete(int) => format!("{int}"),
            }.as_tokenstream());

            items.push(quote! { | });
        }

        items.pop(); // remove extra |

        TokenStream::from_iter(items)
    }

    fn contain(&self, version: u32) -> bool {
        for bounds in self {
            match *bounds {
                Bounds::All => return true,
                Bounds::From(x) => if version >= x { return true },
                Bounds::UpTo(x) => if version <= x { return true },
                Bounds::Range(x, y) => if version >= x && version <= y { return true },
                Bounds::Concrete(x) => if version == x { return true },
            }
        }

        false
    }
}