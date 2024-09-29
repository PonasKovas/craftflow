mod into_traits;
mod packet_read_write;

pub use into_traits::*;
pub use packet_read_write::*;

////////////////////////////////////////////////////////////////
// automatically generated mods from the python script below: //
////////////////////////////////////////////////////////////////
pub mod c2s;
include!(concat!(env!("OUT_DIR"), "/c2s_enum.rs"));
