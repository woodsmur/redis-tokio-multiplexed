use std::env;
use std::process;
use std::time::Duration;

use redis::aio::ConnectionLike;
use redis::ConnectionAddr::Tcp;

enum Mode {
    Default,
    Multiplexed,
}

async fn run<C: ConnectionLike>(mut con: C) -> redis::RedisResult<()> {
    let mut interval = tokio::time::interval(Duration::from_secs(1));
    loop {
        interval.tick().await;
        println!("PING");
        let result: redis::RedisResult<String> = redis::cmd("PING").query_async(&mut con).await;
        println!("Query result: {:?}", result);
    }
}

#[tokio::main]
async fn main() -> redis::RedisResult<()> {
    let mode = match env::args().skip(1).next().as_ref().map(String::as_str) {
        Some("default") => {
            println!("Using default connection mode\n");
            Mode::Default
        }
        Some("multiplexed") => {
            println!("Using multiplexed connection mode\n");
            Mode::Multiplexed
        }
        Some(_) | None => {
            println!("Usage: Pass mode as argument (either 'default' or 'multiplexed')");
            process::exit(1);
        }
    };

    let coninfo = redis::ConnectionInfo {
        addr: Box::new(Tcp("127.0.0.1".to_string(), 6379)),
        db: 0,
        passwd: None, // Some("asdcasc".to_string()),
    };
    let client = redis::Client::open(coninfo).unwrap();
    match mode {
        Mode::Default => run(client.get_async_connection().await?).await?,
        Mode::Multiplexed => run(client.get_multiplexed_tokio_connection().await?).await?,
    };
    Ok(())
}