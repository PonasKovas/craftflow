use std::{
	any::{Any, TypeId},
	collections::BTreeMap,
};

/// A registry of all modules.
pub struct Modules {
	inner: BTreeMap<TypeId, Box<dyn Any>>,
}

// These are not automatically implemented because the Box<dyn Any> might be not Sync + Send
// We can't mark it as Sync + Send there because then we can't downcast it
// (downcast is only implemented for dyn Any, not dyn Any + Sync + Send)
// But in reality we know that all modules are Sync + Send, because we have
// bound checks for it in the module registration method
unsafe impl Sync for Modules {}
unsafe impl Send for Modules {}

impl Modules {
	/// Creates a new empty module registry.
	pub fn new() -> Self {
		Self {
			inner: BTreeMap::new(),
		}
	}

	/// Register a module.
	/// Panics if the module is already registered
	pub fn register<M: Any + Sync + Send>(&mut self, module: M) {
		if self
			.inner
			.insert(TypeId::of::<M>(), Box::new(module))
			.is_some()
		{
			panic!("module already registered");
		}
	}

	/// Try to get a reference to a specific module, if it is registered
	pub fn try_get<M: Any>(&self) -> Option<&M> {
		self.inner
			.get(&TypeId::of::<M>())
			.expect("module not registered")
			.downcast_ref::<M>()
	}
	/// Try to get a mutable reference to a specific module, if it is registered
	pub fn try_get_mut<M: Any>(&mut self) -> Option<&mut M> {
		self.inner
			.get_mut(&TypeId::of::<M>())
			.expect("module not registered")
			.downcast_mut::<M>()
	}
	/// Get a reference to a specific module, panicking if its not registered
	pub fn get<M: Any>(&self) -> &M {
		self.try_get::<M>().expect("module not registered")
	}
	/// Get a mutable reference to a specific module, panicking if its not registered
	pub fn get_mut<M: Any>(&mut self) -> &mut M {
		self.try_get_mut::<M>().expect("module not registered")
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
		let abstracted = Box::new(my_module) as Box<dyn MyTrait + Send + Sync>;

		modules.register(abstracted);

		assert_eq!(modules.get::<Box<dyn MyTrait>>().foo(), 42);
	}
}
