use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Config {
    /// Port to listen on
    #[arg(long, env = "WEB_PORT")]
    pub web_port: u16,

    /// MongoDB URL to connect to
    #[arg(long, env = "MONGO_URL")]
    pub mongo_url: String,
}