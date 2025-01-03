use std::collections::HashMap;
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::thread::spawn;
use tungstenite::accept;
use tungstenite::protocol::WebSocket;
use std::net::TcpStream;

type Clients = Arc<Mutex<HashMap<usize, WebSocket<TcpStream>>>>;

fn main() {
    let server = TcpListener::bind("127.0.0.1:9001").unwrap();
    let clients: Clients = Arc::new(Mutex::new(HashMap::new()));
    let mut id_counter = 0;

    println!("WebSocket server is running on ws://127.0.0.1:9001");

    for stream in server.incoming() {
        let clients = Arc::clone(&clients);
        id_counter += 1;
        let client_id = id_counter;

        spawn(move || {
            let stream = stream.unwrap();
            let  ws = accept(stream).unwrap();
            println!("Client {} connected", client_id);

            {
                let mut clients_lock = clients.lock().unwrap();
                clients_lock.insert(client_id, ws);
            }

            loop {
                let msg = {
                    let mut clients_lock = clients.lock().unwrap();
                    let ws = clients_lock.get_mut(&client_id).unwrap();
                    match ws.read() {
                        Ok(msg) if msg.is_text() || msg.is_binary() => msg,
                        Err(e) => {
                            println!("Client {} disconnected: {}", client_id, e);
                            clients_lock.remove(&client_id);
                            break;
                        }
                        _ => continue,
                    }
                };

                // Broadcast the message to all other clients
                let mut  clients_lock = clients.lock().unwrap();
                for (&id, client) in clients_lock.iter_mut() {
                    if id != client_id {
                        if let Err(e) = client.send(msg.clone()) {
                            println!("Failed to send to {}: {}", id, e);
                        }
                    }
                }
            }
        });
    }
}
