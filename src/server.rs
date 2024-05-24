use std::{
    io::Error,
    sync::{Arc, Mutex},
};

use tokio::{io::AsyncWriteExt, net::TcpListener};

use crate::{
    client::{self, ClientWriter},
    message::{self, Message},
};

pub struct Server<'a> {
    addr: &'a str,
    clients: Arc<Mutex<Vec<ClientWriter>>>,
}

impl<'a> Server<'a> {
    pub async fn new(addr: &'a str) -> Result<(Arc<Mutex<Server<'a>>>, TcpListener), Error> {
        let listener = TcpListener::bind(addr).await?;

        Ok((
            Arc::new(Mutex::new(Server {
                addr,
                clients: Arc::new(Mutex::new(Vec::new())),
            })),
            listener,
        ))
    }

    pub fn get_addr(&self) -> &'a str {
        self.addr
    }

    pub fn get_clients(&self) -> &Arc<Mutex<Vec<ClientWriter>>> {
        &self.clients
    }

    pub fn add_client(&mut self, new_client: ClientWriter) {
        let mut clients = self
            .clients
            .lock()
            .unwrap_or_else(|clients| clients.into_inner());

        clients.push(new_client);
    }

    pub fn remove_client(&mut self, exit_client: Arc<Mutex<ClientWriter>>) {
        let exit_client = exit_client
            .lock()
            .unwrap_or_else(|client| client.into_inner());

        let mut clients = self
            .clients
            .lock()
            .unwrap_or_else(|clients| clients.into_inner());

        clients.retain(|client| client.get_addr() != exit_client.get_addr());
    }

    pub async fn send_to_all(&self, message: Message) {
        println!("{}", message.message);
        for client in self.clients.lock().unwrap().iter_mut() {
            (*client)
                .get_stream()
                .write(message.message.as_bytes())
                .await
                .unwrap();
        }
    }
}
