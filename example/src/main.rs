use craftflow::CraftFlow;
use craftflow_protocol_abstract::c2s::AbLoginStart;
use craftflow_protocol_core::text;
use simple_ping::SimplePing;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	tracing_subscriber::fmt()
		// ... add configuration
		.init();

	let mut craftflow = CraftFlow::new();

	SimplePing::new()
		.set_description(text!(
			"                ♦ CraftFlow ♦",
			color = "white",
			bold
		))
		.register(&mut craftflow);

	craftflow
		.reactor
		.add_handler::<AbLoginStart, _>(|_ctx, (conn_id, packet)| {
			println!("{} {:?}", conn_id, packet);
			std::ops::ControlFlow::Continue((conn_id, packet))
		});

	// Login::default().register(&mut craftflow);

	craftflow.run().await
}
