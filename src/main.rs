use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::broadcast,
};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:8888").await.unwrap();

    let (tx, _) = broadcast::channel(10);

    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();

        let tx = tx.clone();
        let mut rx = tx.subscribe();

        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();

            let mut reader = BufReader::new(reader);

            loop {
                let mut line = String::new();
                tokio::select! {
                    size = reader.read_line(&mut line) => {
                        if size.unwrap() == 0 {
                            break;
                        }
                        tx.send((line, addr)).unwrap();
                    }

                    pack = rx.recv() => {
                        let pack = pack.unwrap();
                        if pack.1 != addr {
                            writer.write_all(pack.0.as_bytes()).await.unwrap();
                        }
                    }
                }
            }
        });
    }
}
