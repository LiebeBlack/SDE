use sha2::{Sha256, Digest};
use escuela_shared::AppResult;
use std::io::Read;

pub fn calculate_sha256(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    hex::encode(result)
}

pub fn calculate_sha256_from_file<R: Read>(reader: &mut R) -> AppResult<String> {
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 8192];
    
    loop {
        let bytes_read = reader.read(&mut buffer)
            .map_err(|e| escuela_shared::AppError::IoError(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Error al leer archivo para calcular hash: {}", e)
            )))?;
        
        if bytes_read == 0 {
            break;
        }
        
        hasher.update(&buffer[..bytes_read]);
    }
    
    let result = hasher.finalize();
    Ok(hex::encode(result))
}

pub fn verify_integrity(data: &[u8], expected_hash: &str) -> bool {
    let calculated_hash = calculate_sha256(data);
    calculated_hash == expected_hash
}

pub fn hash_password(password: &str) -> AppResult<String> {
    use argon2::{
        password_hash::{PasswordHasher, SaltString},
        Argon2
    };
    use rand_core::OsRng;

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)
        .map_err(|e| escuela_shared::AppError::InternalError(format!("Error hashing password: {}", e)))?
        .to_string();
        
    Ok(password_hash)
}

pub fn verify_password(password: &str, hash: &str) -> AppResult<bool> {
    use argon2::{
        password_hash::{PasswordHash, PasswordVerifier},
        Argon2
    };

    let parsed_hash = PasswordHash::new(hash)
        .map_err(|e| escuela_shared::AppError::InternalError(format!("Invalid password hash format: {}", e)))?;
        
    Ok(Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok())
}
