use std::os::unix::net::UnixDatagram;
use std::fs;

const SOCKET_PATH: &str = "/tmp/latency_test.sock";

fn main() -> std::io::Result<()> {
    println!("Iniciando servidor...");

    if fs::metadata(SOCKET_PATH).is_ok() {
        fs::remove_file(SOCKET_PATH)?;
        println!("Socket antiguo eliminado.");
    }

    // 1. Crear el socket y vincularlo a la ruta del archivo.
    let socket = UnixDatagram::bind(SOCKET_PATH)?;
    println!("Servidor escuchando en: {}", SOCKET_PATH);
    println!("Presiona Ctrl+C para detener.");

    let mut buf = [0u8; 10];
    let response = b"respuesta";

    // 2. Bucle infinito para escuchar permanentemente.
    loop {
        let (bytes_read, peer_addr) = socket.recv_from(&mut buf)?;

        if let Some(client_path) = peer_addr.as_pathname() {      
            socket.send_to(response, client_path)?;
        } else {
            eprintln!("Se recibió un mensaje de un cliente sin ruta.");
        }
    }
}