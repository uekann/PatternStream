pub mod invoice;

use anyhow::Result;
use csv::Reader;
use invoice::InvoiceIterator;
use std::env;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};
use tokio::time::{sleep, Duration};

async fn send_data_to_client(mut stream: TcpStream) {
    let mut invoice_reader = Reader::from_path("data/OnlineRetail.csv").unwrap();
    let invoice_iter = InvoiceIterator::new(&mut invoice_reader);

    for invoice in invoice_iter {
        let invoice = invoice.unwrap();
        let invoice_json = serde_json::to_string(&invoice).unwrap();
        stream.write_all(invoice_json.as_bytes()).await.unwrap();
        stream.write_all(b"\r\n").await.unwrap();
        stream.flush().await.unwrap();
        // sleep(Duration::from_secs(1)).await;
    }

    stream.shutdown().await.unwrap();
}

#[tokio::main]
async fn main() -> Result<()> {
    let stream_ip = env::var("STREAM_SERVER_IP")?;
    let stream_port = env::var("STREAM_SERVER_PORT")?;
    let addr = format!("{}:{}", stream_ip, stream_port);
    let listner = TcpListener::bind(&addr).await?;
    println!("Listening on: {}", addr);

    loop {
        let (stream, addr) = listner.accept().await?;
        println!("Connection from: {}", addr);

        tokio::spawn(async move {
            send_data_to_client(stream).await;
        });
    }
}
