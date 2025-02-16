use foliage_rs::{host::Host, peer::Peer, MyService, OtherService, Tag};
use rand::{Rng, SeedableRng};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::{
    net::{UnixListener, UnixStream},
    sync::Mutex,
};

#[derive(Error, Debug)]
enum PluginServiceError {
    #[error("Error")]
    Error,
}

#[derive(Serialize, Deserialize, Debug)]
enum PluginServiceInput {
    Foo(String),
    Boo(String),
}

#[derive(Serialize, Deserialize, Debug)]
enum PluginServiceOutput {
    Foo(String),
    Boo(String),
}

struct PluginService {
    rand: Mutex<rand::rngs::StdRng>,
}

impl OtherService for PluginService {
    type Request = PluginServiceInput;
    type Response = PluginServiceOutput;
    type ErrorResponse = PluginServiceError;
}

impl MyService for PluginService {
    type Request = PluginServiceInput;
    type Response = PluginServiceOutput;
    type Error = PluginServiceError;

    async fn on_rpc(
        &self,
        tag: Tag,
        request: Self::Request,
    ) -> Result<Self::Response, Self::Error> {
        let success = self.rand.lock().await.random_bool(0.5);
        if success {
            match request {
                PluginServiceInput::Foo(msg) => Ok(Self::Response::Boo(format!(
                    "Well Hello There {}: {}",
                    tag, msg
                ))),
                PluginServiceInput::Boo(msg) => Ok(Self::Response::Foo(format!(
                    "Well Hello There {}: {}",
                    tag, msg
                ))),
            }
        } else {
            Err(PluginServiceError::Error)
        }
    }
}

async fn server() -> Result<(), Box<dyn std::error::Error>> {
    let host = Host::new(
        "foo",
        PluginService {
            rand: Mutex::new(rand::rngs::StdRng::from_os_rng()),
        },
    )
    .await?;

    let peer = host.accept().await?;
    tokio::spawn(async move {
        let _ = server_worker(peer).await;
    });
    // }

    Ok(())
}

async fn server_worker(mut peer: Peer<PluginService, PluginService>) -> Result<(), String> {
    loop {
        let result = peer
            .rpc(PluginServiceInput::Boo("Hello Dear".to_string()))
            .await;
        println!("server {:?}", result);
        // tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }

    // Ok(())
}

async fn client() -> Result<(), Box<dyn std::error::Error>> {
    let peer: Peer<PluginService, PluginService> = Peer::new(
        "foo",
        1,
        PluginService {
            rand: Mutex::new(rand::rngs::StdRng::from_os_rng()),
        },
    )
    .await?;

    let mut join_set = tokio::task::JoinSet::new();

    for _ in 0..20 {
        let mut peer = peer.clone();
        join_set.spawn(async move {
            let count = 100;

            let start = std::time::Instant::now();

            for _ in 0..count {
                let result = peer
                    .rpc(PluginServiceInput::Foo("Hello Darling".to_string()))
                    .await;

                println!("client {:?}", result);
            }

            let end = std::time::Instant::now();
            let duration = (end - start) / count;
            // println!("Duration: {:?}", duration);
        });
    }

    join_set.join_all().await;

    Ok(())
}

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let task1 = tokio::spawn(async move {
        server().await.unwrap();
    });

    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    client().await.unwrap();

    task1.await.unwrap();
}
