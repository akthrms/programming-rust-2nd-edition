use async_chat::utils::{self, ChatResult};
use async_chat::{FromClient, FromServer};
use async_std::io;
use async_std::net;
use async_std::prelude::*;
use async_std::task;
use std::sync::Arc;

async fn send_commands(mut to_server: net::TcpStream) -> ChatResult<()> {
    println!(
        "Commands:\njoin GROUP\npost GROUP MESSAGE...\nType Control-D (on Unix) or Control-Z (on Windows) to close the connection"
    );

    let mut command_lines = io::BufReader::new(io::stdin()).lines();

    while let Some(command_result) = command_lines.next().await {
        let command = command_result?;
        let request = match parse_command(&command) {
            Some(request) => request,
            None => continue,
        };

        utils::send_as_json(&mut to_server, &request).await?;
        to_server.flush().await?;
    }

    Ok(())
}

fn parse_command(line: &str) -> Option<FromClient> {
    let (command, rest) = get_next_token(line)?;

    match command {
        "post" => {
            let (group_name, rest) = get_next_token(rest)?;
            let message = rest.trim_start();

            Some(FromClient::Post {
                group_name: Arc::new(group_name.to_string()),
                message: Arc::new(message.to_string()),
            })
        }
        "join" => {
            let (group_name, rest) = get_next_token(rest)?;

            if rest.trim_start().is_empty() {
                Some(FromClient::Join {
                    group_name: Arc::new(group_name.to_string()),
                })
            } else {
                None
            }
        }
        _ => {
            eprintln!("Unrecognized command: {:?}", line);
            None
        }
    }
}

fn get_next_token(mut input: &str) -> Option<(&str, &str)> {
    input = input.trim_start();

    if input.is_empty() {
        None
    } else {
        match input.find(char::is_whitespace) {
            Some(space) => Some((&input[0..space], &input[space..])),
            None => Some((input, "")),
        }
    }
}

async fn handle_replies(from_server: net::TcpStream) -> ChatResult<()> {
    let buffered = io::BufReader::new(from_server);
    let mut reply_stream = utils::receive_as_json(buffered);

    while let Some(reply) = reply_stream.next().await {
        match reply? {
            FromServer::Message {
                group_name,
                message,
            } => {
                println!("message posted to {}: {}", group_name, message);
            }
            FromServer::Error(message) => {
                println!("error from server: {}", message)
            }
        }
    }

    Ok(())
}

fn main() -> ChatResult<()> {
    let address = std::env::args().nth(1).expect("Usage: client ADDRESS:PORT");

    task::block_on(async {
        let socket = net::TcpStream::connect(address).await?;
        socket.set_nodelay(true)?;

        let to_server = send_commands(socket.clone());
        let from_server = handle_replies(socket);

        from_server.race(to_server).await?;

        Ok(())
    })
}
