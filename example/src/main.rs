use craftflow::{CraftFlow, add_callback};
use craftflow_protocol::c2s::login::LoginStart;
use login::Login;
use simple_ping::SimplePing;
use smallbox::SmallBox;
use text::text;
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	tracing_subscriber::fmt()
		.with_line_number(true)
		.with_env_filter(
			EnvFilter::builder()
				.with_default_directive(LevelFilter::INFO.into())
				.from_env_lossy(),
		)
		.init();

	let mut craftflow = CraftFlow::new();

	SimplePing::new()
		.set_description(text!(
			"                ♦ CraftFlow ♦",
			color = "white",
			bold
		))
		.register(&mut craftflow);

	add_callback!(craftflow.reactor, LoginStart => "printer" => |_ctx, (conn_id, packet)| SmallBox::new(async move {
		println!("{} {:?}", conn_id, packet);
		std::ops::ControlFlow::Continue(())
	}));

	Login::default().register(&mut craftflow);

	info!("Starting CraftFlow");

	craftflow.run().await
}
