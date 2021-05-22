use std::net::ToSocketAddrs;
use tokio::net::TcpSocket;

#[tokio::main]
async fn main() {
	let logger = simple_logger::SimpleLogger::new();
	logger.init().unwrap();

	let mut addrs = ("echo.websocket.org", 443).to_socket_addrs().unwrap();
	let addr = addrs.next().unwrap();
	println!("{:?} {:?}", addr, addrs.collect::<Vec<_>>());
	let sock = TcpSocket::new_v4().unwrap();
	let stream = sock.connect(addr).await.unwrap();
	println!("{:?}", stream);

	
	let (str, resp) = tokio_tungstenite::client_async("wss://echo.websocket.org/", stream)
		.await
		.unwrap_or_else(|e| panic!("error {0:?}: {} ", e));
	println!("resp: {:?}", resp);
	println!("str: {:?}", str);
}
