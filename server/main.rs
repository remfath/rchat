use std::io::{ErrorKind, Read, Write};
use std::net::TcpListener;
use std::sync::mpsc;
use std::thread;

use rchat::{LOCAL, MSG_SIZE, sleep};

fn main() {
    let server = TcpListener::bind(LOCAL).expect(&format!("Listening at {} failed", LOCAL));
    println!("Server listening at {}...", LOCAL);
    server.set_nonblocking(true).expect("Set non-blocking failed");

    let mut clients = vec![];
    let (tx, rx) = mpsc::channel::<String>();
    loop {
        if let Ok((mut socket, addr)) = server.accept() {
            println!("Client {} connected", addr);

            let tx = tx.clone();
            clients.push(socket.try_clone().expect("Failed to clone client"));

            thread::spawn(move || loop {
                let mut buffer = vec![0; MSG_SIZE];
                match socket.read_exact(&mut buffer) {
                    Ok(_) => {
                        let msg = buffer.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                        let msg = String::from_utf8(msg).expect("Invalid utf8 message");
                        println!("{}: {:?}", addr, msg);
                        tx.send(msg).expect("Filed to send message");
                    }
                    Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
                    Err(_) => {
                        println!("Close connection with: {}", addr);
                        break;
                    }
                }
                sleep();
            });
        }

        if let Ok(msg) = rx.try_recv() {
            clients = clients.into_iter().filter_map(|mut client| {
                let mut buffer = msg.clone().into_bytes();
                buffer.resize(MSG_SIZE, 0);
                client.write_all(&buffer).map(|_| client).ok()
            }).collect::<Vec<_>>();
        }

        sleep();
    }
}
