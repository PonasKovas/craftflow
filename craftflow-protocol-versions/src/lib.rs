mod into_traits;
mod packet_read_write;

pub use into_traits::*;
pub use packet_read_write::*;

include!(concat!(env!("OUT_DIR"), "/enum_destructure_macro.rs"));

////////////////////////////////////////////////////////////////
// automatically generated mods from the python script below: //
////////////////////////////////////////////////////////////////
pub mod c2s;
include!(concat!(env!("OUT_DIR"), "/c2s_enum.rs"));
pub mod s2c;
include!(concat!(env!("OUT_DIR"), "/s2c_enum.rs"));
