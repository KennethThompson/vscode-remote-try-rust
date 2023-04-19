use include_crypt::{include_crypt, EncryptedFile};

static FILE: EncryptedFile = include_crypt!("assets/file.txt");

fn main() {
    let decrypted = FILE.decrypt();
    let decrypted_str = FILE.decrypt_str();
}