//! Code generation related stuff

include! {"define_type_macro.rs"}

const _: () = {
	// compile example.rs to make sure its valid rust
	mod example {
		use crate::datatypes::*;
		use crate::{Error, MCPRead, MCPWrite, Result};

		include! {"../generator/gen/example_code.rs"}
	}
};
