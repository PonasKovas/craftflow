use craftflow::{add_callback, packet_events::C2SAbLoginStartEvent, CraftFlow};
use craftflow_protocol_core::text;
use login::Login;
use simple_ping::SimplePing;
use smallbox::SmallBox;
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

	add_callback!(craftflow.reactor, C2SAbLoginStartEvent => "printer" => |_ctx, (conn_id, packet)| SmallBox::new(async move {
		println!("{} {:?}", conn_id, packet);
		std::ops::ControlFlow::Continue(())
	}));

	Login::default().register(&mut craftflow);

	info!("Starting CraftFlow");

	craftflow.run().await
}
