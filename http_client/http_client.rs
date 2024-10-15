use std::io::{Read, Write};
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    // 1. Establish a TCP connection
    let mut stream = TcpStream::connect("httpbin.org:80")?;

    // 2. Format the HTTP GET request
    let request = format!(
        "GET /get HTTP/1.1\r\n\
        Host: httpbin.org\r\n\
        User-Agent: RustHTTPClient/1.0\r\n\
        Accept: application/json\r\n\
        Connection: close\r\n\
        \r\n"
    );

    // 3. Send the request
    stream.write_all(request.as_bytes())?;

    // 4. Read the response
    let mut response = String::new();
    stream.read_to_string(&mut response)?;

    // 5. Print the response
    println!("Response:\n{}", response);

    let parts: Vec<&str> = response.split("\r\n\r\n").collect();

    if parts.len() >= 2 {
        println!("Headers: \n{}", parts[0]);
        println!("Body: \n{}", parts[1]);
    }

    Ok(())
}
