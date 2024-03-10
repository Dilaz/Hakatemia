use std::collections::BTreeMap;
use std::io::{prelude::*, BufReader};
use std::net::{Shutdown, TcpStream};
use std::time::Duration;
use clap::Parser;
use regex::Regex;

const DEFAULT_RESULT_PORT: u16 = 6665;
const DEFAULT_SERVICE_PORT: u16 = 6666;
const ASCII_ETX: u8 = 0x03;
const CHUNK_SIZE: usize = 8;
const ALLOWED_CHARS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-_+.,";

#[derive(Parser)]
#[command(version, about, long_about = None, name = "Crackme", author = "Risto \"Dilaz\" Viitanen")]
struct Args {
    /// Target IP address
    #[arg(short, long)]
    target_ip: String,

    /// Secret message port
    #[arg(short, long, default_value_t = DEFAULT_RESULT_PORT)]
    secret_message_port: u16,

    /// Encryption service port
    #[arg(short, long, default_value_t = DEFAULT_SERVICE_PORT)]
    encryption_service_port: u16,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    // Read the secret message from TCP socket
    let message = get_secret(&args.target_ip, args.secret_message_port)?;

    println!("Received: {}", message);

    // Parse the secret message from the received message
    let secret = parse_secret(&message);
    println!("Secret: {}", secret);

    // Get the key from the encryption service
    let key = get_key(&args.target_ip, args.encryption_service_port)?;

    // Decrypt the secret message using the key-map
    let solved_message = decrypt_message(&secret, &key);

    println!("Solved message: {}", solved_message);

    Ok(())
}

fn decrypt_message(secret: &str, key: &BTreeMap<String, char>) -> String {
    let mut decrypted = String::new();
    // Split the secret into chunks of CHUNK_SIZE and use the key-map to decrypt the message
    for chunk in secret.as_bytes().chunks(CHUNK_SIZE).filter_map(|chunk| std::str::from_utf8(chunk).ok()).collect::<Vec<&str>>() {
        decrypted.push(*key.get(&chunk.to_string()).unwrap());
    }

    decrypted
}

fn get_key(target_ip: &str, port: u16) -> Result<BTreeMap<String, char>, std::io::Error> {
    let mut stream = TcpStream::connect(format!("{}:{}", target_ip, port))?;
    println!("Connected to {}:{}!", target_ip, DEFAULT_SERVICE_PORT);
    stream.set_read_timeout(Some(Duration::from_secs(1)))?;
    
    // Read until empty
    let mut buffer = BufReader::new(&stream);
    let mut message = vec![];
    match buffer.read_until(ASCII_ETX,&mut message) {
        Ok(_) => {},
        Err(e) => {
            println!("Error: {}", e);
        }
    }

    println!("Received: {}", std::str::from_utf8(&message).unwrap());

    stream.write(ALLOWED_CHARS.as_bytes())?;

    // Read answer
    let mut buffer = BufReader::new(&stream);
    message.clear();
    match buffer.read_until(ASCII_ETX, &mut message) {
        Ok(_) => {},
        Err(e) => {
            println!("Error: {}", e);
        }
    }
    stream.shutdown(Shutdown::Both)?;

    // Split the message into chunks of CHUNK_SIZE and use the ALLOWED_CHARS to create the key-map
    let mut key = BTreeMap::new();
    let chunked_message = message
        .chunks(CHUNK_SIZE)
        .filter_map(|chunk| std::str::from_utf8(chunk).ok())
        .collect::<Vec<&str>>();

    for (i, chunk) in chunked_message.iter().enumerate() {
        if chunk == &"\n" {
            break;
        }
        key.insert(chunk.to_string(), ALLOWED_CHARS.chars().nth(i).unwrap());
    }

    Ok(key)
}

fn get_secret(target_ip: &str, port: u16) -> Result<String, std::io::Error> {
    let stream = TcpStream::connect(format!("{}:{}", target_ip, port))?;
    stream.set_read_timeout(Some(Duration::from_secs(1)))?;
    println!("Connected to {}:{}!", target_ip, port);
    let mut buffer = BufReader::new(&stream);
    let mut message = vec![];
    match buffer.read_until(ASCII_ETX, &mut message) {
        Ok(_) => (),
        Err(e) => println!("Error: {}", e),
    }
    stream.shutdown(Shutdown::Both)?;

    Ok(std::str::from_utf8(&message).unwrap().to_string())
}

fn parse_secret(message: &str) -> String {
    // Get the first string between [] that has at least 100 characters between them
    let re = Regex::new(r"\[(?<message>[^\]]{100,})\]").unwrap();
    let Some(captures) = re.captures(message) else {
        panic!("No secret found in message: {}", message);
    };

    captures.name("message").unwrap().as_str().to_string()
}
