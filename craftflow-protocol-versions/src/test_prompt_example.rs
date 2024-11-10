use craftflow_nbt::DynNBT;
use craftflow_protocol_core::datatypes::*;
use craftflow_protocol_core::*;
use shallowclone::ShallowClone;
use std::borrow::Cow;

// This tests whether the LLM prompt example code is actually correct for the current codebase

include!("../prompt_example_code.rs");
