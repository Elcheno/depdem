use std::env;

pub struct PathModel {
    pub dir_path: Option<String>,
    pub private_key_file: Option<String>,
    pub public_key_file: Option<String>,
    pub private_key_file_path: Option<String>,
    pub public_key_file_path: Option<String>,
}

impl PathModel {
    pub fn all_paths() -> Result<PathModel, String> {
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

        Ok(PathModel {
            dir_path: Some(dir_path),
            private_key_file: Some(private_key_file),
            public_key_file: Some(public_key_file),
            private_key_file_path: Some(private_key_file_path),
            public_key_file_path: Some(public_key_file_path),
        })
    }

    pub fn get_dir_path() -> Option<String> {
        let dir_path = match env::var("SECRET_PATH") {
            Ok(path) => path,
            Err(_) => {
                println!("Error to load SECRET_PATH env variable");
                return None;
            }
        };
        Some(dir_path)
    }

    pub fn get_private_key_file_path() -> Option<String> {
        let private_key_file_path = match env::var("PRIVATE_KEY_FILE") {
            Ok(path) => path,
            Err(_) => {
                println!("Error to load PUBLIC_KEY_FILE env variable");
                return None;
            }
        };
        Some(private_key_file_path)
    }

    pub fn get_public_key_file_path() -> Option<String> {
        let public_key_file_path = match env::var("PUBLIC_KEY_FILE") {
            Ok(path) => path,
            Err(_) => {
                println!("Error to load PUBLIC_KEY_FILE env variable");
                return None;
            }
        };
        Some(public_key_file_path)
    }
}
