use std::thread; // use thread library
use std::net::{TcpListener, TcpStream, Shutdown}; // use net library
use std::io::{Read, Write}; // use io library

fn handle_client(mut stream: TcpStream) { // function declare
    let mut data = [0 as u8; 50]; // using 50 byte buffer
    while match stream.read(&mut data) { // use while to create a loop, to keep reading tcp message
        Ok(size) => { // when get some data
            if size != 0 { // ignore when size == 0
                let s = String::from_utf8_lossy(&data); // convert data to String
                println!("Message received: {}", s); // print data string
            }
            stream.write(&data[0..size]).unwrap(); // echo everything, send data back
            true // return true
        },
        Err(_) => { // when get an error
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap()); // print error log
            stream.shutdown(Shutdown::Both).unwrap(); // close connection
            false // return false
        }
    } {}
}

fn main() { // function entry
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap(); // bind localhost 3333 port to get a listener
    println!("Server listening on port 3333"); // if this is printed, connction is successful
    for stream in listener.incoming() { // use for to create a loop, to keep listening tcp connections
        match stream { // match stream to decide which part of code will be run
            Ok(stream) => { // is valid stream
                println!("New connection: www {}", stream.peer_addr().unwrap()); // print new connection log
                thread::spawn(move|| { // accept connections and process them, spawning a new thread for each one
                    handle_client(stream) // process data
                });
            }
            Err(e) => { // has error
                println!("Error: {}", e); // connection failed, print error
            }
        }
    }
    drop(listener); // close the socket server
}