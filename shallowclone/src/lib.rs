//! [`ShallowClone`] trait to make it easier to work with copy-on-write values efficiently.
//!
//! This is basically the same as the standard [`Clone`], except that it's optimized for copy-on-write
//! values so that they're not cloned. Shallow cloning a [`Cow`][std::borrow::Cow] will always produce
//! a `Borrowed` variant, either referencing the original value if it was `Owned`, or the value that the
//! original value referenced if it already was `Borrowed`.
//!
//! This crate also introduces a [`MakeOwned`] trait which is the opposite of [`ShallowClone`].
//! It takes any value that implements the trait and returns an equivalent which is `'static` - no references,
//! completely self-sufficient.
//!
//! Additionally this crate introduces two replacements for the standard [`Cow<'a, T>`][std::borrow::Cow]:
//!  - [`CoCow<'a, T>`][CoCow] which is a general replacement for the standard [`Cow`][std::borrow::Cow],
//!  - [`CoCowSlice<'a, T>`][CoCowSlice] which is a specialised replacement for [`Cow<'a, [T]>`][std::borrow::Cow].
//!
//! These types are covariant over `T`, which solves some problems if your `T` contains references.
//! In most cases you probably won't need them, standard [`Cow`][std::borrow::Cow] works perfectly for
//! things like [`Cow<'a, str>`][std::borrow::Cow] or [`Cow<'a, [u8]>`][std::borrow::Cow], but if your
//! `T` contains references, the standard [`Cow`][std::borrow::Cow] will not let you subtype them
//! after shallow cloning, and you will end up with 2 different lifetimes.
//! [`CoCow`] and [`CoCowSlice`] solve this problem.

mod cows;
mod makeowned;
mod shallowclone;

pub use cows::{CoCow, CoCowSlice};
pub use makeowned::MakeOwned;
pub use shallowclone::ShallowClone;

/// Automatically derives the [`MakeOwned`] trait
///
/// ## `#[shallowclone(skip)]` attribute
///
/// You can use this attribute on generics (type or lifetime) to not place [`MakeOwned`] bounds on them,
/// if your type requires so.
///
/// ```
/// # use std::marker::PhantomData;
/// # use shallowclone::MakeOwned;
/// #[derive(MakeOwned, Clone)]
/// struct MyStruct<#[makeowned(skip)] T> {
///     // No need to place the bounds on T, since it's inside the PhantomData
///     phantom: PhantomData<T>,
/// }
/// ```
pub use shallowclone_derive::MakeOwned;
/// Automatically derives the [`ShallowClone`] trait
///
/// ## `#[shallowclone(skip)]` attribute
///
/// You can use this attribute on generics (type or lifetime) to not place [`ShallowClone`] bounds on them,
/// if your type requires so.
///
/// ```
/// # use std::marker::PhantomData;
/// # use shallowclone::ShallowClone;
/// #[derive(ShallowClone)]
/// struct MyStruct<#[shallowclone(skip)] T> {
///     // No need to place the bounds on T, since it's inside the PhantomData
///     phantom: PhantomData<T>,
/// }
/// ```
pub use shallowclone_derive::ShallowClone;
