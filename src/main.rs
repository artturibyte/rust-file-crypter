mod cli_parser;
mod crypt;
mod encrypt;
use crypt::crypt_file;
use encrypt::decrypt_file;
use cli_parser::Commands;
use clap::Parser;

fn main() {
    let cli = cli_parser::Cli::parse();
    
    match &cli.command {
        Commands::Crypt(args) => {
            println!("Crypting file in path: {:?}", &args.path);
            crypt_file(&args.path, &args.password);
        } ,
        Commands::Decrypt(args) => {
            println!("Decrpyting file in path: {:?}", &args.path);
            decrypt_file(&args.path, &args.password);
        },
    }
}
