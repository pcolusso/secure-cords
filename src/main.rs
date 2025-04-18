use anyhow::Result;
use clap::Parser;
use home::home_dir;
use servers::Server;
use ssm::Session;
use std::path::PathBuf;

mod servers;
mod ssm;
mod ui;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Config {
    #[arg(short, long)]
    connections_file: Option<PathBuf>,
}

type Uhh = (Session, Server, bool);

#[tokio::main]
async fn main() -> Result<()> {
    let Config { connections_file } = Config::parse();

    let connections_file = match connections_file {
        None => home_dir()
            .expect("Can't get home dir.")
            .join("Documents/jobs.json"),
        Some(f) => f,
    };

    let servers = servers::load(&connections_file).await?;
    let mapped: Vec<Uhh> = servers
        .into_iter()
        .map(|s| {
            (
                Session::new(
                    s.identifier.clone(),
                    s.env.clone(),
                    s.host_port,
                    s.dest_port,
                ),
                s,
                false,
            )
        })
        .collect();

    ui::run(mapped, connections_file).await?;

    Ok(())
}
