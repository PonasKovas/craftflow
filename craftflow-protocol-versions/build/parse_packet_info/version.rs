#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Version(pub u32);

impl Version {
	pub fn mod_name(&self) -> String {
		format!("v{:05}", self.0)
	}
	pub fn caps_mod_name(&self) -> String {
		format!("V{:05}", self.0)
	}
}
