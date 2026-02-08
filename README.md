# Arquitectura-ipc

## Reto de Latencia Mínima: Solución con IPC
Este proyecto es una implementación en Rust de un sistema de estímulo-respuesta con una latencia inferior a un milisegundo. 
La solución se basa en una arquitectura de Comunicación entre Procesos (IPC) utilizando Sockets de Dominio Unix.

## Arquitectura
La arquitectura elegida consiste en dos procesos separados que se ejecutan en la misma máquina: un Servidor y un Cliente.

Servidor (server): Un programa que escucha en un punto de comunicación conocido. Al recibir un mensaje, responde inmediatamente.
Cliente (client): Un programa que envía un mensaje al servidor, espera la respuesta.
La comunicación entre ellos utiliza Sockets de Dominio Unix (UnixDatagram). 

![Arquitectura IPC](/image/arquitectura.png "Arquitectura IPC")

## Estructura de Archivos
El proyecto está organizado como un workspace de Cargo:
```
.
├── Cargo.toml         # Archivo del workspace
├── client/            # Crate del cliente
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
└── server/            # Crate del servidor
    ├── Cargo.toml
    └── src/
        └── main.rs
```

##  Prerrequisitos
### Instalación Rust
Para compilar y ejecutar este proyecto, necesitas tener instalado Rust. 
Ejecuta el siguiente comando que descarga y ejecuta el script de instalación oficial de Rust.
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Presiona Enter para elegir la opción por defecto (1) Proceed with installation (default).
Cierra tu terminal y vuelve a abrirla.

Verifica la instalación:
```
cargo --version
```

### Cómo Compilar y Ejecutar
Ve a la raíz del proyecto (IPC/).

1. Compilar el Proyecto
Compila ambos, el cliente y el servidor, en modo release. Este modo activa optimizaciones cruciales para el rendimiento.
```
cargo build --release
```
2. Ejecutar el Servidor
Abre una terminal y ejecuta el servidor. Se quedará escuchando indefinidamente.
```
./target/release/server
```
Deberías ver la siguiente salida:
```
Iniciando servidor...
Servidor escuchando en: /tmp/latency_test.sock
Presiona Ctrl+C para detener.
```
3. Ejecutar el Cliente
Abre una segunda terminal y ejecuta el cliente. Este enviará una gran cantidad de estímulos y medirá la latencia de cada uno.
```
./target/release/client
```
## Resultados Esperados
Ver file cargado en la carpeta `/resultados` 

----------------------------------------------------
