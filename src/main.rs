use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
    time::Duration,
};

fn main() {
    let server = TcpListener::bind("localhost:6969").expect("ERROR: Could not run server");
    let streams = Arc::new(Mutex::new(Vec::new()));

    for stream in server.incoming() {
        match stream {
            Ok(stream) => {
                println!("New Connection: {}", stream.peer_addr().unwrap());

                let stream = Arc::new(Mutex::new(stream));

                streams.lock().unwrap().push(stream.clone());

                let streams_clone = streams.clone();

                std::thread::spawn(move || {
                    stream.lock().unwrap().set_nonblocking(true).unwrap();
                    handle_stream(stream, streams_clone);
                });
            }
            Err(_) => (),
        }
    }
}

fn handle_stream(
    user_stream: Arc<Mutex<TcpStream>>,
    streams: Arc<Mutex<Vec<Arc<Mutex<TcpStream>>>>>,
) {
    let mut buffer_name = [0; 128];
    let mut buffer = [0; 2048];
    let mut buffer_size;
    let user_addr = user_stream.lock().unwrap().peer_addr().unwrap();
    let mut response = String::new();
    let name;

    user_stream
        .lock()
        .unwrap()
        .write("Digite seu apelido: ".as_bytes())
        .unwrap();

    loop {
        std::thread::sleep(Duration::from_millis(100));

        buffer_size = match user_stream.lock().unwrap().read(&mut buffer_name) {
            Ok(size) => size,
            Err(err) => {
                if err.kind() == std::io::ErrorKind::WouldBlock {
                    continue;
                }
                return;
            }
        };
        break;
    }

    name = std::str::from_utf8(&buffer_name[0..buffer_size])
        .unwrap()
        .trim();

    loop {
        std::thread::sleep(Duration::from_millis(100));

        buffer_size = match user_stream.lock().unwrap().read(&mut buffer) {
            Ok(size) => size,
            Err(err) => {
                if err.kind() == std::io::ErrorKind::WouldBlock {
                    continue;
                }
                break;
            }
        };

        if buffer_size <= 2 {
            continue;
        }

        response.clear();
        response.push_str(
            format!(
                "[{name}] {}",
                std::str::from_utf8(&buffer[0..buffer_size]).unwrap()
            )
            .as_str(),
        );

        for stream in streams.lock().unwrap().iter() {
            let mut stream = stream.lock().unwrap();

            match stream.peer_addr() {
                Ok(addr) => {
                    if addr == user_addr {
                        continue;
                    }
                }
                Err(_) => {
                    continue;
                }
            }

            match stream.write(response.as_bytes()) {
                Ok(_) => (),
                Err(err) => {
                    if err.kind() == std::io::ErrorKind::WouldBlock {
                        continue;
                    }
                    break;
                }
            }
        }
    }

    println!(
        "Connection Lost: {}",
        user_stream.lock().unwrap().peer_addr().unwrap()
    );
}
