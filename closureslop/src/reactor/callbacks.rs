use petgraph::graph::{DiGraph, NodeIndex};
use std::any::Any;

pub(super) struct Callbacks {
	pub(super) event_name: &'static str,
	order: Vec<NodeIndex>,
	graph: DiGraph<Callback, ()>,
}

pub(super) struct Callback {
	pub(super) id: String,
	pub(super) callback: Box<dyn Any + Send + Sync>,
	pub(super) must_come_after: Vec<String>,
	pub(super) must_come_before: Vec<String>,
}

impl Callbacks {
	pub(super) fn new(name: &'static str) -> Self {
		Self {
			event_name: name,
			order: Vec::new(),
			graph: DiGraph::new(),
		}
	}
	/// Returns the callbacks in order, generating the order if it hasn't been generated yet
	pub(super) fn in_order(&self) -> impl Iterator<Item = &Callback> {
		self.order.iter().map(|index| &self.graph[*index])
	}
	/// Panics if theres already a callback with the same id
	pub(super) fn add_callback(&mut self, callback: Callback) {
		// make sure there isn't already a callback with this id
		for other in self.graph.node_weights() {
			if other.id == callback.id {
				panic!(
					"Callback with id {} already exists for this event",
					callback.id
				);
			}
		}

		self.graph.add_node(callback);
		self.generate_order();
	}
	/// Generates the order of callbacks to be executed
	/// panics if the order is cyclic
	pub(super) fn generate_order(&mut self) {
		// regenerate all edges and then sort the graph topologically, saving the order
		self.graph.clear_edges();

		for callback_id in self.graph.node_indices() {
			for dep_type in ["after", "before"] {
				let deps = if dep_type == "after" {
					&self.graph[callback_id].must_come_after
				} else {
					&self.graph[callback_id].must_come_before
				}
				.clone();

				for dep in deps {
					// not existing dependencies are silently ignored
					// this is order-relations, not actual dependencies, you shouldn't use this to specify
					// required dependencies.
					if let Some(target_id) = self.find_with_id(&dep) {
						if dep_type == "after" {
							self.graph.add_edge(callback_id, target_id, ());
						} else {
							self.graph.add_edge(target_id, callback_id, ());
						}
					}
				}
			}
		}

		// all edges are added, now sort the graph
		self.order = match petgraph::algo::toposort(&self.graph, None) {
			Ok(order) => order,
			Err(e) => panic!("Cyclic dependencies detected in callbacks order: {e:?}"),
		};
		self.order.reverse();
	}
	/// Finds a callback with the given id, returning whether it's async and the index
	fn find_with_id(&self, id: &str) -> Option<NodeIndex> {
		for callback in self.graph.node_indices() {
			if &self.graph[callback].id == id {
				return Some(callback);
			}
		}
		None
	}
}
