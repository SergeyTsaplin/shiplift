use shiplift::Docker;
use tokio::prelude::Future;

fn main() {
    env_logger::init();
    let docker = Docker::new();
    let fut = docker
        .containers()
        .list(&Default::default())
        .map(|containers| {
            for c in containers {
                println!("container -> {:#?}", c)
            }
        })
        .map_err(|e| eprintln!("Error: {}", e));

    tokio::run(fut);
}
