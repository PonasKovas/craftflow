use craftflow::CraftFlow;
use craftflow_protocol_abstract::c2s::AbLoginStart;
use craftflow_protocol_core::text;
use login::Login;
use simple_ping::SimplePing;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	tracing_subscriber::fmt()
		.with_line_number(true)
		// .with_max_level(level_filters::LevelFilter::TRACE)
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

	Login::default().register(&mut craftflow);

	info!("Starting CraftFlow");

	craftflow.run().await
}
