/**
 * @title Rust 3 way handshake
 * @description This is simple example of 3 way handshake in Rust
 * @author Takahashi Akari
 * @license MIT License copyright 2022 Takahashi Akari
 * @version v0.1.0
 * @date 2022-12-15
 */

use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() {
    let mut input = String::new();
    println!("Enter server IP address and port");
    println!("Example: 192.168.1.120:8080");
    println!("Enter: ");
    std::io::stdin().read_line(&mut input).unwrap();
    let server_ip_port = input.trim();

    // server
    let listener = TcpListener::bind(server_ip_port).unwrap();
    // accept connections and process them serially
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                // connection succeeded
                thread::spawn(move|| {
                    // connection succeeded
                    handle_client(stream)
                });
            }
            Err(e) => {
                // connection failed
                println!("Error: {}", e);
            }
        }
    }
    // close the socket server
    drop(listener);
}

fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 50]; // using 50 byte buffer
    while match stream.read(&mut data) {
        Ok(size) => {
            // echo everything!
            stream.write(&data[0..size]).unwrap();
            true
        }
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(std::net::Shutdown::Both).unwrap();
            false
        }
    } {}
}
