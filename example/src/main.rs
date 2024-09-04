use craftflow::CraftFlow;
use craftflow_protocol::text;
use login::Login;
use simple_ping::SimplePing;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	tracing_subscriber::fmt()
		// ... add configuration
		.init();

	let mut craftflow = CraftFlow::new();

	SimplePing::new()
		.set_description(
			text!("This is an example server! ", color = "gold")
				+ text!("CONNECT ", bold, color = "aqua")
				+ text!("NOW!", bold, color = "green"),
		)
		.register(&mut craftflow);

	Login::default().register(&mut craftflow);

	craftflow.run().await
}
