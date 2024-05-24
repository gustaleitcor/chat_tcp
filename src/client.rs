use std::{mem::MaybeUninit, net::SocketAddr};

use tokio::{
    net::tcp::{OwnedReadHalf, OwnedWriteHalf},
    sync::mpsc::{Receiver, Sender},
};

use crate::message::Message;

pub struct ClientWriter {
    addr: SocketAddr,
    stream: OwnedWriteHalf,
}

impl ClientWriter {
    pub fn new(addr: SocketAddr, stream: OwnedWriteHalf) -> ClientWriter {
        ClientWriter { addr, stream }
    }

    pub fn get_addr(&self) -> &SocketAddr {
        &self.addr
    }

    pub fn get_stream(&mut self) -> &mut OwnedWriteHalf {
        &mut self.stream
    }
}

pub struct ClientReader<'a> {
    pub name: Box<MaybeUninit<&'a str>>,
    addr: SocketAddr,
    stream: OwnedReadHalf,
    sender: Sender<Message>,
}

impl<'a> ClientReader<'a> {
    pub fn new(
        addr: SocketAddr,
        stream: OwnedReadHalf,
        sender: Sender<Message>,
    ) -> ClientReader<'a> {
        ClientReader {
            name: Box::new_uninit(),
            addr,
            stream,
            sender,
        }
    }

    pub fn get_addr(&self) -> &SocketAddr {
        &self.addr
    }

    pub fn get_stream(&mut self) -> &mut OwnedReadHalf {
        &mut self.stream
    }

    pub fn get_sender(&self) -> &Sender<Message> {
        &self.sender
    }
}
