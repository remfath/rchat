use std::io::{self, ErrorKind, Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::{self, TryRecvError};
use std::thread;

use rchat::{LOCAL, MSG_SIZE, sleep};

fn main() {
    let mut client = TcpStream::connect(LOCAL).expect(&format!("Connect to {} failed", LOCAL));
    client.set_nonblocking(true).expect("Failed to set non-blocking");

    let (tx, rx) = mpsc::channel::<String>();

    thread::spawn(move || loop {
        let mut buffer = vec![0; MSG_SIZE];
        match client.read_exact(&mut buffer) {
            Ok(_) => {
                let msg = buffer.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                println!("Message received: {:?}", msg);
            }
            Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
            Err(_) => {
                println!("Connection with server was severed");
                break;
            }
        }

        match rx.try_recv() {
            Ok(msg) => {
                let mut buffer = msg.clone().into_bytes();
                buffer.resize(MSG_SIZE, 0);
                client.write_all(&buffer).expect("Write to socket failed");
                println!("Message sent: {:?}", msg);
            }
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => break,
        }

       sleep();
    });

    println!("Write a message: ");
    loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).expect("Read message failed");
        let msg = buffer.trim().to_string();
        if msg == ":quit" || tx.send(msg).is_err() {
            break;
        }
    }

    println!("Bye!");
}