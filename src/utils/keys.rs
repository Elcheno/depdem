use crate::utils::transform::transform_vec_to_string;

use crate::models::path_model::PathModel;
use rand::rngs::OsRng;
use rsa::pkcs8::{DecodePrivateKey, DecodePublicKey};
use rsa::PublicKey;
use rsa::{
    pkcs8::EncodePrivateKey, pkcs8::EncodePublicKey, PaddingScheme, RsaPrivateKey, RsaPublicKey,
};
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

pub fn generate_keys() -> Result<(), String> {
    let paths = PathModel::all_paths()?;

    let mut rng = OsRng;
    let private_key = RsaPrivateKey::new(&mut rng, 2048).expect("Error to generate RSA 2048 bits");
    let public_key = RsaPublicKey::from(&private_key);

    if !Path::new(&paths.dir_path.clone().unwrap()).exists() {
        fs::create_dir_all(&paths.dir_path.clone().unwrap())
            .map_err(|_| "Error to create directory")?;
    }

    // Guarda la clave privada en formato PEM
    let mut private_file = File::create(&paths.private_key_file_path.unwrap())
        .expect("Error to create private_key.pem file");

    let private_pem = private_key
        .to_pkcs8_pem(rsa::pkcs8::LineEnding::LF)
        .expect("Error to generate private key");

    private_file
        .write_all(private_pem.as_bytes())
        .expect("Error to write private key in file");

    // Guarda la clave pública en formato PEM
    let mut public_file = File::create(&paths.public_key_file_path.unwrap())
        .expect("Error to create public_key.pem file");

    let public_pem = public_key
        .to_public_key_pem(rsa::pkcs8::LineEnding::LF)
        .expect("Error generate public key");
    public_file
        .write_all(public_pem.as_bytes())
        .expect("Error to write public key in file");

    println!("Generated and saved 'private_key.pem' and 'public_key.pem'");
    Ok(())
}

pub fn load_keys() -> Result<String, String> {
    let paths = PathModel::all_paths()?;

    if !Path::new(&paths.private_key_file_path.unwrap()).exists()
        || !Path::new(&paths.public_key_file_path.unwrap()).exists()
    {
        return Err(String::from("Error to load keys"));
    } else {
        Ok("Keys loaded successfully".to_string())
    }
}

pub fn verify_keys(public_key: &String) -> Result<bool, String> {
    let private_key_file_path = PathModel::get_private_key_file_path();

    // Cargar la clave privada
    let private_key_data = fs::read(private_key_file_path.unwrap())
        .map_err(|_| "Error to read private key".to_string())?;

    let private_key_string = transform_vec_to_string(&private_key_data);

    let private_key = RsaPrivateKey::from_pkcs8_pem(&private_key_string)
        .map_err(|_| "Error to load private key".to_string())?;

    // Cargar la clave pública
    let public_key = RsaPublicKey::from_public_key_pem(&public_key)
        .map_err(|_| "Error to load public key".to_string())?;

    // Mensaje de prueba
    let message = b"Hey depdem!";

    // Firmar el mensaje con la clave privada
    let signature = private_key
        .sign(PaddingScheme::new_pkcs1v15_sign(None), &message[..])
        .map_err(|_| "Error to sign message".to_string())?;

    // Verificar la firma con la clave pública
    public_key
        .verify(
            PaddingScheme::new_pkcs1v15_sign(None),
            &message[..],
            &signature,
        )
        .map_err(|_| "Verify signature failed".to_string())?;

    Ok(true)
}
