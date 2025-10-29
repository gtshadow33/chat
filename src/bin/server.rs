use anyhow::Result;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
};

#[tokio::main]
async fn main() -> Result<()> {
    // Crear el listener
    let listener = TcpListener::bind("127.0.0.1:6142").await?;
    println!("Esperando conexión...");

    // Aceptar una sola conexión
    let (socket, addr) = listener.accept().await?;
    println!("Cliente conectado desde {}", addr);

    // Dividir el socket en lectura y escritura
    let (reader, mut writer) = socket.into_split();

    // Envolver el reader en un BufReader y obtener líneas
    let mut client_reader = BufReader::new(reader).lines();

    // Tarea que lee los mensajes del cliente
    let read_task = tokio::spawn(async move {
        while let Ok(Some(line)) = client_reader.next_line().await {
            println!("Cliente: {}", line);
        }
        println!("Conexión cerrada por el cliente.");
    });

    // Tarea que lee del teclado y envía al cliente
    let stdin = tokio::io::stdin();
    let mut stdin_reader = BufReader::new(stdin).lines();
    let write_task = tokio::spawn(async move {
        while let Ok(Some(line)) = stdin_reader.next_line().await {
            if writer.write_all(line.as_bytes()).await.is_err() {
                println!("Error al enviar mensaje, conexión cerrada.");
                break;
            }
            if writer.write_all(b"\n").await.is_err() {
                println!("Error al enviar mensaje, conexión cerrada.");
                break;
            }
        }
    });

    // Esperar a que las tareas terminen
    let _ = tokio::join!(read_task, write_task);

    Ok(())
}
