use std::net::TcpListener;
use std::thread::spawn;
use tungstenite::accept;

fn main() {
    let ip = TcpListener::bind("127.0.0.1:9001").unwrap();

    for stream in ip.incoming(){
        spawn(move || {
            let mut ws = accept(stream.unwrap()).unwrap();
            loop {
                let msg = ws.read().unwrap();

                if msg.is_binary()|| msg.is_text(){
                    ws.send(msg).unwrap();
                }
            }
        });
    }
}
