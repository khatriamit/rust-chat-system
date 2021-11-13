
use tokio::
{io::{AsyncBufReadExt, AsyncWriteExt, BufReader}, net::TcpListener, sync::broadcast};

#[tokio::main]
async fn main() {
    // creating TCP server to listen from tokio
    let listener = TcpListener::bind("localhost:8000").await.unwrap();
    
    let (tx, _rx) = broadcast::channel(10) ;
    // accepting multiple clients
    loop{
        let (mut socket, addr) = listener.accept().await.unwrap();

        let tx = tx.clone();
        let mut rx = tx.subscribe();
        // spawning the task to prevent blocking of code 
        // allowing multiple client to send message
        tokio::spawn(async move{

            let (reader, mut write) = socket.split();

            let mut reader = BufReader ::new(reader);
            let mut line = String::new();

            // allowing user to write multiple messages
            loop{
                tokio::select! {
                    result = reader.read_line(&mut line) =>{
                        if result.unwrap() == 0{
                            break;
                        }
                        tx.send((line.clone(), addr)).unwrap();
                        line.clear();
                    }
                    result = rx.recv() =>{
                        let (msg, other_attr)=result.unwrap();
                        if addr != other_attr{
                            write.write_all(msg.as_bytes()).await.unwrap();
                    }
                    }
                }
            }
        });
    }
}
