mod deep;

use once_cell::sync::Lazy;
use std::env;
use std::{collections::HashMap, sync::Arc, u16};
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
use surrealdb_migrations::MigrationRunner;
use testcontainers::clients::{self, Cli};
use testcontainers::{core::WaitFor, Container, Image, ImageArgs};

pub static DOCKER: Lazy<Cli> = Lazy::new(clients::Cli::default);

const NAME: &str = "surrealdb/surrealdb";
const TAG: &str = "v1.2.1";

const USERNAME: &str = "root";
const PASSWORD: &str = "root";
const NAMESPACE: &str = "app";
const DATABASE: &str = "app";

const DB_PORT: u16 = 8000;

#[tokio::test]
async fn test_one() -> anyhow::Result<()> {
    SurrealDbTestContext::init().await.unwrap();

    Ok(())
}

pub struct SurrealDbTestContext<'a> {
    pub connection: Arc<Lazy<Surreal<Client>>>,
    /// The docker test container
    pub container: Container<'a, SurrealDbImage>,
    /// The port the container listens on the host network
    pub container_host_port: u16,
}

impl SurrealDbTestContext<'_> {
    pub async fn init() -> anyhow::Result<Self> {
        let container = DOCKER.run(SurrealDbImage::new());
        let container_host_port = container.get_host_port_ipv4(DB_PORT);

        let config_file_path = env::current_dir()
            .unwrap()
            .parent()
            .unwrap()
            .join(".surrealdb");

        println!("Use .surrealdb file from: {}", config_file_path.display());

        let connection: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);

        connection
            .connect::<Ws>(format!("127.0.0.1:{}", container_host_port))
            .await
            .expect("Failed to connect");

        connection
            .signin(Root {
                username: USERNAME,
                password: PASSWORD,
            })
            .await
            .expect("Failed to sign in");

        connection
            .use_ns(NAMESPACE)
            .use_db(DATABASE)
            .await
            .expect("Failed to use ns/db");

        MigrationRunner::new(&connection)
            .use_config_file(&config_file_path)
            .up()
            .await
            .expect("Failed to apply migrations");

        println!("Started SurrealDB on: 127.0.0.1:{}", container_host_port);

        Ok(SurrealDbTestContext {
            connection: Arc::new(connection),
            container,
            container_host_port,
        })
    }
}

#[derive(Debug, Default)]
pub struct SurrealDbImage {
    vars: HashMap<String, String>,
}

impl SurrealDbImage {
    pub fn new() -> Self {
        let mut vars = HashMap::new();
        vars.insert("SURREAL_CAPS_ALLOW_ALL".to_owned(), "true".to_owned()); // -A
        vars.insert("SURREAL_AUTH".to_owned(), "true".to_owned()); // --auth
        vars.insert("SURREAL_USER".to_owned(), USERNAME.to_owned()); // --username
        vars.insert("SURREAL_PASS".to_owned(), PASSWORD.to_owned()); // --password

        Self { vars }
    }
}

#[derive(Debug, Default, Clone)]
pub struct SurrealDbImageArgs;

impl ImageArgs for SurrealDbImageArgs {
    fn into_iterator(self) -> Box<dyn Iterator<Item = String>> {
        Box::new(
            vec![
                "start".to_owned(),
                "--log=trace".to_owned(),
                "memory".to_owned(),
            ]
            .into_iter(),
        )
    }
}

impl Image for SurrealDbImage {
    type Args = SurrealDbImageArgs;

    fn name(&self) -> String {
        NAME.to_owned()
    }

    fn tag(&self) -> String {
        TAG.to_owned()
    }

    fn ready_conditions(&self) -> Vec<testcontainers::core::WaitFor> {
        vec![WaitFor::message_on_stderr("Started node agent".to_owned())]
    }

    fn env_vars(&self) -> Box<dyn Iterator<Item = (&String, &String)> + '_> {
        Box::new(self.vars.iter())
    }

    fn expose_ports(&self) -> Vec<u16> {
        vec![DB_PORT]
    }
}
