use std::env;
use std::net::TcpStream;
use std::time::Duration;
use std::thread::sleep;
use std::process::exit;
use std::process::Command;

fn start_command(command: &String) {
    Command::new("sh").arg("-c").arg(command).spawn().expect("failed to execute command");
}

fn detect_tcp(proto: &String, max: i32) -> Result<i32, String> {
    let stream = TcpStream::connect(proto);
    if stream.is_err() {
        if max > 0 {
            sleep(Duration::from_secs(1));
            return detect_tcp(proto, max - 1)
        } else {
            return Err(String::from("Connection timed out"));
        }
    } else {
        return Ok(max.clone());
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let tcp_endpoint: String = args[1].clone();
    let mut command: String = String::from("");
    if args.len() > 2 {
        command = args[2].clone();
        if command == "--" {
            command = args[3..].join(" ");
        }
    }

    println!("Waiting for TCP connection to {} for 30s...", tcp_endpoint);

    match detect_tcp(&tcp_endpoint, 30) {
        Ok(time_left) => {
            println!("Connected after {}s", 30 - time_left);
            if command != "" {
                start_command(&command);
            }
            exit(0)
        },
        Err(error) => {
            println!("{}", error);
            exit(1);
        }
    }
}
