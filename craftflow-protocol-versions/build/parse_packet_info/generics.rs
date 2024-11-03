/// Only lifetimes
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Generics(pub Vec<String>);

impl Generics {
	pub fn new() -> Self {
		Self(Vec::new())
	}
	pub fn union(&self, other: &Self) -> Self {
		let mut generics = self.0.clone();
		for g in &other.0 {
			if !generics.contains(g) {
				generics.push(g.clone());
			}
		}
		Self(generics)
	}
	pub fn as_str(&self) -> String {
		format!("<{}>", self.0.join(", "))
	}
	/// fills self generics with generics from other, using 'static if not present
	pub fn fill_with(&self, other: &Self) -> Self {
		let mut generics = Generics::new();
		for g in &self.0 {
			if other.0.contains(g) {
				generics.0.push(g.clone());
			} else {
				generics.0.push("'static".to_string());
			}
		}

		generics
	}
}
