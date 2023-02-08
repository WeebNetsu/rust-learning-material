mod lib;

use std::io::prelude::*;
use std::net::{TcpListener, TcpStream}; // for read streams
use std::{thread, time::Duration};

use lib::ThreadPool;

fn main() {
    let port: String = String::from("7878");
    let entry_point: String = String::from("localhost:") + &port;
    // bind works the same as 'new', it will return TcpListener instance
    // we use bind because connection to a port in networking is called
    // binding to a port
    match TcpListener::bind(&entry_point) {
        // if no error occurs
        Ok(listener) => {
            let pool = ThreadPool::new(4); // limit the number of threads

            println!("Listenting on http://{}", entry_point);

            // incoming of course listens for incomming connections
            for stream in listener.incoming() {
                let stream = stream.unwrap();

                // this will spawn a thread for each connection
                // this is not a good way to handle multiple
                // connections at the same time, since DDoS attacks
                // can be done by sending a lot of connections
                // we can of course use a ThreadPool to limit
                // the number of threads that can be used
                /* thread::spawn(|| {
                    // but as an example, /sleep does not make
                    // the whole server wait for a response
                    // anymore
                    handle_connection(stream);
                }); */

                // using a ThreadPool instead
                pool.execute(|| {
                    handle_connection(stream);
                });
            }
        },
        // if an error occurs
        Err(_) => {
            println!("Could not bund to port {}. Is another service already running?", port);
        }
    }
}

// stream needs to be mutable because in this case reading might
// change the value of stream
fn handle_connection(mut stream: TcpStream) {
    // create a buffer on the stack to hold the data that is read
    // we allow it to store 1024 bytes of data, enough for our simple server
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    // since buffer contains bytes, we need
    // to provide it bytes when we compare
    let get_home = b"GET / HTTP/1.1\r\n";
    let get_about = b"GET /about HTTP/1.1\r\n";
    // to simulate a slow server, we'll sleep for 5 seconds
    let get_sleep = b"GET /sleep HTTP/1.1\r\n";
    let ok_response = "HTTP/1.1 200 OK";

    let (status_line, filename) = if buffer.starts_with(get_home) {
        (ok_response, "pages/index.html")
    } else if buffer.starts_with(get_about) {
        (ok_response, "pages/about.html")
    } else if buffer.starts_with(get_sleep) {
        // we'll sleep for 5 seconds
        // you'll notice without threads, if we have 2
        // tabs open in our browser, one trying to
        // access /sleep and the other /about
        // both will wait for 5 seconds, not just
        // /sleep
        thread::sleep(Duration::from_secs(5));
        (ok_response, "pages/index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "pages/404.html")
    };

    let response_contents = std::fs::read_to_string(filename).unwrap();

    // https://doc.rust-lang.org/book/ch20-01-single-threaded.html#a-closer-look-at-an-http-request
    // this will send a 200 OK status message as a response
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        response_contents.len(),
        response_contents
    );

    // we convert the bytes into a string, the from_utf8_lossy
    // converts &[u8] to String... lossy means that any invalid
    // characters will  be replaced with ? symbol
    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    // we need to convert strings to bytes to send data as a response
    stream.write(response.as_bytes()).unwrap();

    // flush() will stip the program from continuing until the
    // bytes have been written to the connection
    stream.flush().unwrap();
}
