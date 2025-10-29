use anyhow::Result;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
};

#[tokio::main]
async fn main() -> Result<()> {
    let socket = TcpStream::connect("127.0.0.1:6142").await?;
    println!("Conectado al servidor. Puedes escribir mensajes en cualquier momento.");

    let (reader, mut writer) = socket.into_split();
    let mut server_reader = BufReader::new(reader).lines();

    // Tarea para leer mensajes del servidor todo el tiempo
    let read_task = tokio::spawn(async move {
        while let Ok(Some(line)) = server_reader.next_line().await {
            println!("Servidor: {}", line);
        }
        println!("Servidor desconectado.");
    });

    // Tarea para escribir al servidor todo el tiempo
    let stdin = tokio::io::stdin();
    let mut stdin_reader = BufReader::new(stdin).lines();
    let write_task = tokio::spawn(async move {
        while let Ok(Some(line)) = stdin_reader.next_line().await {
            if writer.write_all(line.as_bytes()).await.is_err() {
                println!("Error al enviar, conexión cerrada.");
                break;
            }
            if writer.write_all(b"\n").await.is_err() {
                println!("Error al enviar, conexión cerrada.");
                break;
            }
        }
    });

    let _ = tokio::join!(read_task, write_task);
    Ok(())
}
