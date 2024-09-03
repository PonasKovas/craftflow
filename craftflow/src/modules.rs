use std::{
	any::{Any, TypeId},
	collections::BTreeMap,
};

/// A registry of all modules.
pub struct Modules {
	inner: BTreeMap<TypeId, Box<dyn Any>>,
}

impl Modules {
	/// Creates a new empty module registry.
	pub fn new() -> Self {
		Self {
			inner: BTreeMap::new(),
		}
	}

	/// Register a module.
	/// Panics if the module is already registered
	pub fn register<M: Any>(&mut self, module: M) {
		if self
			.inner
			.insert(TypeId::of::<M>(), Box::new(module))
			.is_some()
		{
			panic!("module already registered");
		}
	}

	/// Get a reference to a specific module.
	/// Panics if the module is not registered
	pub fn get<M: Any>(&self) -> &M {
		self.inner
			.get(&TypeId::of::<M>())
			.expect("module not registered")
			.downcast_ref::<M>()
			.unwrap()
	}

	/// Get a mutable reference to a specific module.
	/// Panics if the module is not registered
	pub fn get_mut<M: Any>(&mut self) -> &mut M {
		self.inner
			.get_mut(&TypeId::of::<M>())
			.expect("module not registered")
			.downcast_mut::<M>()
			.unwrap()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_modules_simple() {
		let mut modules = Modules::new();

		modules.register(42);
		assert_eq!(*modules.get::<i32>(), 42);

		*modules.get_mut::<i32>() = 43;
		assert_eq!(*modules.get::<i32>(), 43);
	}

	#[test]
	fn test_modules_trait() {
		trait MyTrait {
			fn foo(&self) -> i32;
		}

		struct MyModule;
		impl MyTrait for MyModule {
			fn foo(&self) -> i32 {
				42
			}
		}

		let mut modules = Modules::new();

		let my_module = MyModule;
		let abstracted = Box::new(my_module) as Box<dyn MyTrait>;

		modules.register(abstracted);

		assert_eq!(modules.get::<Box<dyn MyTrait>>().foo(), 42);
	}
}
