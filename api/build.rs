use openssl::rsa::Rsa;
use std::{env, error::Error, fs::File, io::Write, path::Path};

fn main() -> Result<(), Box<dyn Error>> {
    let path = Path::new(&env::var("CARGO_MANIFEST_DIR")?).join("src/.db_key");

    if !path.exists() {
        let key = Rsa::generate(4096)?.private_key_to_pem()?;
        File::create(&path)?.write_all(&key)?;
    }

    Ok(())
}
