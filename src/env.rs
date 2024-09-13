use clap::Parser;
use dotenv::dotenv;

#[derive(Parser, Debug, Clone)]
pub struct Env {
    #[clap(env, default_value = "127.0.0.1")]
    pub host: String,

    #[clap(env, default_value = "3333")]
    pub port: u16,
}

impl Env {
    pub fn from_env() -> Self {
        dotenv().ok();
        Env::parse()
    }
}
