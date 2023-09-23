use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    
    /*
      The bind function in this scenario works like the new function in that it will 
      return a new TcpListener instance. The function is called bind because, 
      in networking, connecting to a port to listen to is known as “binding to a port.
      TODO: add error handling
    */
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    /*
      The incoming method on TcpListener returns an iterator that gives us a sequence of streams 
      (more specifically, streams of type TcpStream)
     
      A single stream represents an open connection between the client and the server. 
      A connection is the name for the full request and response process in which a client connects 
      to the server, the server generates a response, and the server closes the connection.
    */
    for streams in listener.incoming() {
        let stream = streams.unwrap();

        handle_connection(stream);
    }
}

/*
  BufReader implements the std::io::BufRead trait, which provides the lines method. 
  The lines method returns an iterator of Result<String, std::io::Error> 
  by splitting the stream of data whenever it sees a newline byte. 
  To get each String, we map and unwrap each Result
*/
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    
    if request_line == "GET / HTTP/1.1" {
        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string("hello.html").unwrap();
        let length = contents.len();

        let response =
            format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        /*
        The write_all method on stream takes a &[u8] 
        and sends those bytes directly down the connection.
        */
        stream.write_all(response.as_bytes()).unwrap();    
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let contents = fs::read_to_string("404.html").unwrap();
        let length = contents.len();

        let response = format!(
            "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
        );

        stream.write_all(response.as_bytes()).unwrap();
    }


    
}
