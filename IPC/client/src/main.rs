use std::os::unix::net::UnixDatagram;
use std::time::{Instant, Duration};
use std::fs;
use std::process;

const SERVER_SOCKET_PATH: &str = "/tmp/latency_test.sock";
const NUM_ITERATIONS: u32 = 100_000;

fn main() -> std::io::Result<()> {
    let client_socket_path = format!("/tmp/latency_client.{}.sock", process::id());

    if fs::metadata(&client_socket_path).is_ok() {
        fs::remove_file(&client_socket_path)?;
    }

    // 1. Vincular el socket del cliente a su propia dirección para poder recibir la respuesta.
    let socket = UnixDatagram::bind(&client_socket_path)?;

    socket.connect(SERVER_SOCKET_PATH)?;

    println!("Cliente listo. Enviando {} estímulos a {}...", NUM_ITERATIONS, SERVER_SOCKET_PATH);

    let mut latencies: Vec<Duration> = Vec::with_capacity(NUM_ITERATIONS as usize);
    let stimulus = b"stimulus";
    let mut buf = [0u8; 10];

    // Calentamiento: realiza algunas iteraciones para que el SO "despierte".
    for _ in 0..1000 {
        socket.send(stimulus)?;
        socket.recv(&mut buf)?;
    }

    // 2. El bucle de medición.
    for _ in 0..NUM_ITERATIONS {
        let start = Instant::now();

        socket.send(stimulus)?;    
        socket.recv(&mut buf)?; 

        let elapsed = start.elapsed(); 
        latencies.push(elapsed);
    }

    println!("Medición completada.");

    // 3. Análisis de resultados.
    let total_duration: Duration = latencies.iter().sum();
    let avg_latency = total_duration / NUM_ITERATIONS;
    let min_latency = latencies.iter().min().unwrap();
    let max_latency = latencies.iter().max().unwrap();

    println!("\n--- Resultados de Latencia (viaje de ida y vuelta) ---");
    println!("Pruebas realizadas: {}", NUM_ITERATIONS);
    println!("Latencia Mínima:  {:.3} µs", min_latency.as_micros());
    println!("Latencia Promedio: {:.3} µs", avg_latency.as_micros());
    println!("Latencia Máxima:   {:.3} µs", max_latency.as_micros());
    println!("----------------------------------------------------");

    // Objetivo del reto: < 1 milisegundo (1000 microsegundos)
    if avg_latency.as_micros() < 1000 {
        println!("\n¡ÉXITO! La latencia promedio está muy por debajo de 1 milisegundo.");
    } else {
        println!("\nFALLO. La latencia promedio superó 1 milisegundo.");
    }

    // Limpiar el archivo del socket del cliente al terminar.
    fs::remove_file(&client_socket_path)?;

    Ok(())
}