use tokio::{
	net::{TcpListener, TcpStream},
	sync::mpsc::Sender,
};

pub async fn listener_task(
	listener: TcpListener,
	new_conn_sender: Sender<TcpStream>,
) -> anyhow::Result<()> {
	loop {
		let (stream, _) = listener.accept().await?;
		new_conn_sender.send(stream).await?;
	}
}
