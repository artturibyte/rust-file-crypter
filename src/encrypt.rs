use magic_crypt::{new_magic_crypt, MagicCryptTrait};


pub fn decrypt_file(file_path: &std::path::PathBuf, passwrd: &str) {
    let mc = new_magic_crypt!(passwrd, 256);
    if !file_path.exists() {
        println!("Filepath does not exists!");
        return;
    }
    let content = std::fs::read_to_string(file_path).expect("Could not read file");
    match mc.decrypt_base64_to_string(&content) {
        Ok(contents) => {std::fs::write(file_path, contents).unwrap();}, 
        Err(e) => {println!("{:?}", e)}
    }
    
}

