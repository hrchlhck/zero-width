use clap::{Parser, Subcommand};

#[derive(Debug, Clone, Subcommand)]
pub enum Program {
    /// Hide a secret message within a cover text
    Hide {
        #[arg(short, long)]
        /// Cover text for the secret message
        cover: String,

        #[arg(short, long)]
        /// The actual secret message to be hidden
        message: String,
    },

    /// Reveal a potential secret message from a cover text
    Reveal {

        #[arg(short, long)]
        /// The payload containing the potential hidden message
        payload: String
    }
}

#[derive(Parser, Debug)]
#[command(version="0.1.0", about, long_about = None)]
#[command(author="Pedro Rocha Horchulhack <pedro@prhrck.com>")]
pub struct App {
    #[command(subcommand)]
    pub action: Program,

    #[arg(short, long, required=false, default_value_t="output.txt".to_string())]
    pub path: String
}