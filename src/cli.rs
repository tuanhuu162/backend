use structopt::StructOpt;

#[derive(StructOpt, Debug, Clone)]
#[structopt(name = "todolist")]
pub struct Opt {
    
    /// Port to listen to 
    #[structopt(short, long, env = "PORT", default_value = "8000")]
    pub port: u16,
    
    /// HOST 
    #[structopt(long, env = "HOST", default_value = "localhost")]
    pub host: String
}
