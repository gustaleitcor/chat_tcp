#![feature(new_uninit)]
#![feature(async_closure)]

mod client;
mod message;
mod server;

use std::time::Duration;

use tokio::{io::AsyncReadExt, sync::mpsc::channel};

use crate::client::{ClientReader, ClientWriter};
use crate::message::Message;
use crate::server::Server;

#[tokio::main]
async fn main() {
    let (server, listener) = Server::new("localhost:5050")
        .await
        .expect("ERROR: Could not run server");

    println!("  --- Server started ---");
    println!("Listening to: {}", server.lock().unwrap().get_addr());

    let server_clone = server.clone();
    let (server_sender, mut server_receiver) = channel::<Message>(1024);

    let incoming_thread = tokio::spawn(async move {
        loop {
            let conn = listener.accept().await;

            match conn {
                Ok((stream, client_addr)) => {
                    println!("New connection: {client_addr}");

                    let (read_half, write_half) = stream.into_split();
                    let client_sender_clone = server_sender.clone();

                    let client_writer = ClientWriter::new(client_addr, write_half);
                    let client_reader =
                        ClientReader::new(client_addr, read_half, client_sender_clone);

                    server_clone.lock().unwrap().add_client(client_writer);

                    tokio::spawn(async move {
                        handle_client(client_reader).await;
                    });
                }
                Err(_) => (),
            }
        }
    });

    let server_clone = server.clone();

    loop {
        let message = server_receiver.recv().await.unwrap();
        server_clone
            .clone()
            .lock()
            .unwrap()
            .send_to_all(message)
            .await;
    }

    incoming_thread.await.unwrap();
}

async fn handle_client<'a>(mut client: ClientReader<'a>) {
    let mut buffer = [0; 2048];
    let mut buffer_size;

    loop {
        std::thread::sleep(Duration::from_millis(100));

        buffer_size = match client.get_stream().read(&mut buffer).await {
            Ok(size) => size,
            Err(err) => {
                if err.kind() == std::io::ErrorKind::WouldBlock {
                    continue;
                }
                return;
            }
        };

        let msg = String::from_utf8(buffer[0..buffer_size].to_vec()).unwrap();

        client
            .get_sender()
            .send(Message {
                message: msg,
                adresser: *client.get_addr(),
            })
            .await
            .unwrap();
    }
}
