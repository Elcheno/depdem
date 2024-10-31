use rand::rngs::OsRng;
use rsa::pkcs8::{DecodePrivateKey, DecodePublicKey};
use rsa::PublicKey;
use rsa::{
    pkcs8::EncodePrivateKey, pkcs8::EncodePublicKey, PaddingScheme, RsaPrivateKey, RsaPublicKey,
};
use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

pub fn generate_keys() -> Result<(), String> {
    let dir_path = match env::var("SECRET_PATH") {
        Ok(path) => path,
        Err(_) => return Err(String::from("Error to load SECRET_PATH env variable")),
    };

    let private_key_file = match env::var("PRIVATE_KEY_FILE") {
        Ok(path) => path,
        Err(_) => return Err(String::from("Error to load PRIVATE_KEY_FILE env variable")),
    };

    let public_key_file = match env::var("PUBLIC_KEY_FILE") {
        Ok(path) => path,
        Err(_) => return Err(String::from("Error to load PUBLIC_KEY_FILE env variable")),
    };

    let private_key_file_path = format!("{}{}", dir_path, private_key_file);
    let public_key_file_path = format!("{}{}", dir_path, public_key_file);

    let mut rng = OsRng;
    let private_key = RsaPrivateKey::new(&mut rng, 2048).expect("Error to generate RSA 2048 bits");
    let public_key = RsaPublicKey::from(&private_key);

    if !Path::new(&dir_path).exists() {
        fs::create_dir_all(&dir_path).map_err(|_| "Error to create directory")?;
    }

    // Guarda la clave privada en formato PEM
    let mut private_file =
        File::create(&private_key_file_path).expect("Error to create private_key.pem file");

    let private_pem = private_key
        .to_pkcs8_pem(rsa::pkcs8::LineEnding::LF)
        .expect("Error to generate private key");

    private_file
        .write_all(private_pem.as_bytes())
        .expect("Error to write private key in file");

    // Guarda la clave pública en formato PEM
    let mut public_file =
        File::create(&public_key_file_path).expect("Error to create public_key.pem file");

    let public_pem = public_key
        .to_public_key_pem(rsa::pkcs8::LineEnding::LF)
        .expect("Error generate public key");
    public_file
        .write_all(public_pem.as_bytes())
        .expect("Error to write public key in file");

    println!("Generated and saved 'private_key.pem' and 'public_key.pem'");
    Ok(())
}

pub fn load_keys() -> Result<(), String> {
    let dir_path = match env::var("SECRET_PATH") {
        Ok(path) => path,
        Err(_) => return Err(String::from("Error to load SECRET_PATH env variable")),
    };

    let private_key_file = match env::var("PRIVATE_KEY_FILE") {
        Ok(path) => path,
        Err(_) => return Err(String::from("Error to load PRIVATE_KEY_FILE env variable")),
    };

    let public_key_file = match env::var("PUBLIC_KEY_FILE") {
        Ok(path) => path,
        Err(_) => return Err(String::from("Error to load PUBLIC_KEY_FILE env variable")),
    };

    let private_key_file_path = format!("{}{}", dir_path, private_key_file);
    let public_key_file_path = format!("{}{}", dir_path, public_key_file);

    if !Path::new(&private_key_file_path).exists() || !Path::new(&public_key_file_path).exists() {
        return Err(String::from("Error to load keys"));
    } else {
        Ok(())
    }
}

pub fn verify_keys(public_key: &String) -> Result<bool, String> {
    // load path files
    let dir_path = match env::var("SECRET_PATH") {
        Ok(path) => path,
        Err(_) => return Err(String::from("Error to load SECRET_PATH env variable")),
    };

    let private_key_file = match env::var("PRIVATE_KEY_FILE") {
        Ok(path) => path,
        Err(_) => return Err(String::from("Error to load PRIVATE_KEY_FILE env variable")),
    };

    let private_key_file_path = format!("{}{}", dir_path, private_key_file);

    // Cargar la clave privada
    let private_key_data = fs::read(private_key_file_path)
        .map_err(|_| "Error al leer la clave privada".to_string())?;

    let private_key_string = String::from_utf8(private_key_data.clone())
        .map_err(|_| "Error al convertir la clave privada a String".to_string())?;

    let private_key = RsaPrivateKey::from_pkcs8_pem(&private_key_string)
        .map_err(|_| "Error al cargar la clave privada".to_string())?;

    // Cargar la clave pública
    let public_key = RsaPublicKey::from_public_key_pem(&public_key)
        .map_err(|_| "Error al cargar la clave pública".to_string())?;

    // Mensaje de prueba
    let message = b"Hey depdem!";

    // Firmar el mensaje con la clave privada
    let signature = private_key
        .sign(PaddingScheme::new_pkcs1v15_sign(None), &message[..])
        .map_err(|_| "Error al firmar el mensaje".to_string())?;

    // Verificar la firma con la clave pública
    public_key
        .verify(
            PaddingScheme::new_pkcs1v15_sign(None),
            &message[..],
            &signature,
        )
        .map_err(|_| "La verificación de la firma falló".to_string())?;

    Ok(true)
}
