use std::ops::ControlFlow;

use craftflow::CraftFlow;
use craftflow_protocol::text;
use simple_ping::SimplePing;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	tracing_subscriber::fmt()
		// ... add configuration
		.init();

	let mut craftflow = CraftFlow::new();

	let mut simple_ping = SimplePing::new();
	simple_ping.set_description(
		text!("This is an example server! ", color = "gold")
			+ text!("CONNECT ", bold, color = "aqua")
			+ text!("NOW!", bold, color = "green"),
	);
	simple_ping.register(&mut craftflow);

	craftflow.run().await
}
