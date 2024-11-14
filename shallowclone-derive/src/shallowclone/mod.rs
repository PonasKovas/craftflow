pub mod attributes;
mod gen_impl;
mod target_type;

pub use gen_impl::gen_impl_shallowclone;
pub use target_type::get_target_type;
