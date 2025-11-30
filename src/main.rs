use ssh2::Session;
use std::net::TcpStream;
use std::io::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ip_addr = "192.168.1.13";
    let user = "littlebuddha";
    let password = "pranavr2002";

    println!("Attempting to connect to {}...", ip_addr);

    // Create TCP connection
    let tcp = TcpStream::connect(format!("{}:22", ip_addr))?;
    
    // Create SSH session
    let mut sess = Session::new()?;
    sess.set_tcp_stream(tcp);
    
    // Perform handshake
    sess.handshake()?;
    
    // Authenticate with username and password
    sess.userauth_password(user, password)?;
    
    // Verify connection by checking if authenticated
    if sess.authenticated() {
        println!("Successfully connected to {} as {}!", ip_addr, user);
    } else {
        return Err("Authentication failed".into());
    }

    Ok(())
}
