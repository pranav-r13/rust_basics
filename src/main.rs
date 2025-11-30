use ssh2::Session;
use std::net::TcpStream;
use std::io::prelude::*;

fn connect_and_execute(
    ip_addr: &str,
    user: &str,
    password: &str,
) -> Result<(), Box<dyn std::error::Error>> {
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
        
        // Execute ls command
        println!("Running 'ls' command...");
        let mut channel = sess.channel_session()?;
        channel.exec("ls")?;
        
        // Read the output
        let mut output = String::new();
        channel.read_to_string(&mut output)?;
        channel.wait_close()?;
        
        // Print the output
        println!("Current directory: {}", output.trim());
        
        // Close the channel
        channel.close()?;
        channel.wait_close()?;
        
        println!("Connection closed.");
    } else {
        return Err("Authentication failed".into());
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ip_addr = "192.168.1.13";
    let user = "username";
    let password = "password";

    connect_and_execute(ip_addr, user, password)?;

    Ok(())
}
