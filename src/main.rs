// 
//
//██▀███   █    ██   ██████ ▄▄▄█████▓ ███▄ ▄███▓ ▄▄▄       ██▓███  
//▓██ ▒ ██▒ ██  ▓██▒▒██    ▒ ▓  ██▒ ▓▒▓██▒▀█▀ ██▒▒████▄    ▓██░  ██▒
//▓██ ░▄█ ▒▓██  ▒██░░ ▓██▄   ▒ ▓██░ ▒░▓██    ▓██░▒██  ▀█▄  ▓██░ ██▓▒
//▒██▀▀█▄  ▓▓█  ░██░  ▒   ██▒░ ▓██▓ ░ ▒██    ▒██ ░██▄▄▄▄██ ▒██▄█▓▒ ▒
//░██▓ ▒██▒▒▒█████▓ ▒██████▒▒  ▒██▒ ░ ▒██▒   ░██▒ ▓█   ▓██▒▒██▒ ░  ░
//░ ▒▓ ░▒▓░░▒▓▒ ▒ ▒ ▒ ▒▓▒ ▒ ░  ▒ ░░   ░ ▒░   ░  ░ ▒▒   ▓▒█░▒▓▒░ ░  ░
//  ░▒ ░ ▒░░░▒░ ░ ░ ░ ░▒  ░ ░    ░    ░  ░      ░  ▒   ▒▒ ░░▒ ░     
//  ░░   ░  ░░░ ░ ░ ░  ░  ░    ░      ░      ░     ░   ▒   ░░       
//   ░        ░           ░                  ░         ░  ░         

// Dependencies
use std::env;
use std::net::{IpAddr, SocketAddr};
use tokio::net::TcpStream;
use tokio::time::{self, Duration};
use color_print::cprintln;


// Port Scanning
async fn is_open(ip: IpAddr, port: u16) -> bool {
    let socket = SocketAddr::new(ip, port);
    match TcpStream::connect(&socket).await {
        Ok(_) => {
            true
        }
        Err(_) => {
            false
        }    
    }
}

async fn scan_ports(ip: IpAddr, start_port: u16, end_port: u16) {
    for port in start_port..=end_port {
        time::sleep(Duration::from_millis(0)).await;
        let ip_clone = ip;
        tokio::spawn(async move {
            if is_open(ip_clone, port).await {
                cprintln!("<green><bold>[!]</bold> Port <bold>{}</bold> is open</green>", port);
            }
        });
    }
}


#[tokio::main]
async fn main() {
    cprintln!("<red>{}</red>", r#"
    ██▀███   █    ██   ██████ ▄▄▄█████▓ ███▄ ▄███▓ ▄▄▄       ██▓███  
    ▓██ ▒ ██▒ ██  ▓██▒▒██    ▒ ▓  ██▒ ▓▒▓██▒▀█▀ ██▒▒████▄    ▓██░  ██▒
    ▓██ ░▄█ ▒▓██  ▒██░░ ▓██▄   ▒ ▓██░ ▒░▓██    ▓██░▒██  ▀█▄  ▓██░ ██▓▒
    ▒██▀▀█▄  ▓▓█  ░██░  ▒   ██▒░ ▓██▓ ░ ▒██    ▒██ ░██▄▄▄▄██ ▒██▄█▓▒ ▒
    ░██▓ ▒██▒▒▒█████▓ ▒██████▒▒  ▒██▒ ░ ▒██▒   ░██▒ ▓█   ▓██▒▒██▒ ░  ░
    ░ ▒▓ ░▒▓░░▒▓▒ ▒ ▒ ▒ ▒▓▒ ▒ ░  ▒ ░░   ░ ▒░   ░  ░ ▒▒   ▓▒█░▒▓▒░ ░  ░
      ░▒ ░ ▒░░░▒░ ░ ░ ░ ░▒  ░ ░    ░    ░  ░      ░  ▒   ▒▒ ░░▒ ░     
      ░░   ░  ░░░ ░ ░ ░  ░  ░    ░      ░      ░     ░   ▒   ░░       
       ░        ░           ░                  ░         ░  ░         
       "#);
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <ip>", args[0]);
        std::process::exit(1);
    }

    let ip: IpAddr = match args[1].parse() {
        Ok(ip) => ip,
        Err(_) => {
            eprintln!("Invalid IP Address");
            std::process::exit(1);
        }  
    };

    let start_port = 1;
    let end_port = 5000;
    let start_time = time::Instant::now();
    scan_ports(ip, start_port, end_port).await;
    let elapsed_time = start_time.elapsed();

    cprintln!("<yellow><bold>[*]</bold> Scanning took: {:?}</yellow>\n", elapsed_time);
    
    cprintln!("<blue><bold>[*]</bold> Scanning Summary:</blue>");
    let is_ssh = is_open(ip, 22);
    let is_http = is_open(ip, 80);
    let is_https = is_open(ip, 443);

    if is_ssh.await {
        cprintln!("<red><bold>Port 22</bold> // <bold>TCP/SSH is active on the server.</bold> // <bold>EXPLOITABLE</bold></red>");
    }

    if is_http.await {
        cprintln!("<red><bold>Port 80</bold> // <bold>HTTP is active on the server.</bold> // <bold>INSECURE/HOSTED_WEBSITE</bold></red>");
    }

    if is_https.await {
        cprintln!("<red><bold>Port 443</bold> // <bold>HTTPS is active on the server.</bold> // <bold>HOSTED_WEBSITE</bold></red>");
    }
    
}