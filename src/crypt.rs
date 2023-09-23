use magic_crypt::{new_magic_crypt, MagicCryptTrait};


pub fn crypt_file(file_path:  &std::path::PathBuf, passwrd: &str) {
    let mc = new_magic_crypt!(passwrd, 256);
    let mut crypted_path = file_path.clone();
    crypted_path.set_extension("crypted");
    if !file_path.exists() {
        println!("Filepath does not exists!");
        return;
    }
    
    let content = std::fs::read_to_string(file_path).expect("Could not read file");
    let new_string = mc.encrypt_str_to_base64(content);
    match std::fs::write(crypted_path, new_string) {
    Ok(_) => { },
    Err(e) => {println!("{:?}", e)}
    }
}

