use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author="Artturi Mård", version)]
/// Crypt and decrypt text files.
pub struct Cli {
   #[command(subcommand)]
   pub command: Commands,
}

#[derive(Args, Debug)]
pub struct DecryptArgs {
   ///Path to file to Decrypt
   pub path: std::path::PathBuf,
   pub password: String
}

#[derive(Args, Debug)]
pub struct CryptArgs { 
   ///Path to file to crypt
   pub path: std::path::PathBuf,
   pub password: String
}


#[derive(Subcommand, Debug)]
pub enum Commands {
   Crypt(CryptArgs),
   Decrypt(DecryptArgs)
}
