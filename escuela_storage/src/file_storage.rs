use std::path::{Path, PathBuf};
use std::fs;
use std::io::Write;
use escuela_shared::{AppResult, AppError};
use escuela_core::security::calculate_sha256;

pub struct FileStorageService {
    base_path: PathBuf,
}

impl FileStorageService {
    pub fn new(base_path: &str) -> AppResult<Self> {
        let base_path = PathBuf::from(base_path);
        
        if !base_path.exists() {
            fs::create_dir_all(&base_path).map_err(|e| {
                AppError::IoError(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Error al crear directorio base de almacenamiento: {}", e),
                ))
            })?;
        }
        
        Ok(FileStorageService { base_path })
    }
    
    pub fn crear_directorio_expediente(&self, expediente_id: &str) -> AppResult<PathBuf> {
        let expediente_dir = self.base_path.join(expediente_id);
        
        if !expediente_dir.exists() {
            fs::create_dir_all(&expediente_dir).map_err(|e| {
                AppError::IoError(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Error al crear directorio del expediente {}: {}", expediente_id, e),
                ))
            })?;
        }
        
        Ok(expediente_dir)
    }
    
    pub fn guardar_archivo(
        &self,
        expediente_id: &str,
        nombre_archivo: &str,
        contenido: &[u8],
    ) -> AppResult<ArchivoGuardado> {
        let expediente_dir = self.crear_directorio_expediente(expediente_id)?;
        
        let hash = calculate_sha256(contenido);
        let extension = Path::new(nombre_archivo)
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("bin");
        
        let nombre_archivo_hash = format!("{}.{}", hash, extension);
        let ruta_completa = expediente_dir.join(&nombre_archivo_hash);
        
        let mut archivo = fs::File::create(&ruta_completa).map_err(|e| {
            AppError::IoError(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Error al crear archivo {}: {}", ruta_completa.display(), e),
            ))
        })?;
        
        archivo.write_all(contenido).map_err(|e| {
            AppError::IoError(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Error al escribir archivo {}: {}", ruta_completa.display(), e),
            ))
        })?;
        
        let ruta_local = ruta_completa
            .to_str()
            .ok_or_else(|| AppError::InternalError("Error al convertir ruta a string".to_string()))?
            .to_string();
        
        Ok(ArchivoGuardado {
            ruta_local,
            hash,
            tamaño_bytes: contenido.len() as u64,
            nombre_archivo_original: nombre_archivo.to_string(),
            nombre_archivo_hash,
        })
    }
    
    pub fn guardar_archivo_con_nombre_personalizado(
        &self,
        expediente_id: &str,
        nombre_archivo: &str,
        contenido: &[u8],
        prefijo: Option<&str>,
    ) -> AppResult<ArchivoGuardado> {
        let expediente_dir = self.crear_directorio_expediente(expediente_id)?;
        
        let hash = calculate_sha256(contenido);
        let extension = Path::new(nombre_archivo)
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("bin");
        
        let hash_corto = &hash[..16];
        let nombre_archivo_hash = match prefijo {
            Some(p) => format!("{}_{}.{}", p, hash_corto, extension),
            None => format!("{}.{}", hash_corto, extension),
        };
        
        let ruta_completa = expediente_dir.join(&nombre_archivo_hash);
        
        let mut archivo = fs::File::create(&ruta_completa).map_err(|e| {
            AppError::IoError(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Error al crear archivo {}: {}", ruta_completa.display(), e),
            ))
        })?;
        
        archivo.write_all(contenido).map_err(|e| {
            AppError::IoError(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Error al escribir archivo {}: {}", ruta_completa.display(), e),
            ))
        })?;
        
        let ruta_local = ruta_completa
            .to_str()
            .ok_or_else(|| AppError::InternalError("Error al convertir ruta a string".to_string()))?
            .to_string();
        
        Ok(ArchivoGuardado {
            ruta_local,
            hash,
            tamaño_bytes: contenido.len() as u64,
            nombre_archivo_original: nombre_archivo.to_string(),
            nombre_archivo_hash,
        })
    }
    
    pub fn leer_archivo(&self, ruta_local: &str) -> AppResult<Vec<u8>> {
        let ruta = PathBuf::from(ruta_local);
        
        if !ruta.exists() {
            return Err(AppError::NotFound(format!("Archivo no encontrado: {}", ruta_local)));
        }
        
        let contenido = fs::read(&ruta).map_err(|e| {
            AppError::IoError(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Error al leer archivo {}: {}", ruta_local, e),
            ))
        })?;
        
        Ok(contenido)
    }
    
    pub fn eliminar_archivo(&self, ruta_local: &str) -> AppResult<()> {
        let ruta = PathBuf::from(ruta_local);
        
        if !ruta.exists() {
            return Err(AppError::NotFound(format!("Archivo no encontrado: {}", ruta_local)));
        }
        
        fs::remove_file(&ruta).map_err(|e| {
            AppError::IoError(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Error al eliminar archivo {}: {}", ruta_local, e),
            ))
        })?;
        
        Ok(())
    }
    
    pub fn eliminar_directorio_expediente(&self, expediente_id: &str) -> AppResult<()> {
        let expediente_dir = self.base_path.join(expediente_id);
        
        if !expediente_dir.exists() {
            return Err(AppError::NotFound(format!("Directorio del expediente no encontrado: {}", expediente_id)));
        }
        
        fs::remove_dir_all(&expediente_dir).map_err(|e| {
            AppError::IoError(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Error al eliminar directorio del expediente {}: {}", expediente_id, e),
            ))
        })?;
        
        Ok(())
    }
    
    pub fn verificar_archivo_existe(&self, ruta_local: &str) -> bool {
        PathBuf::from(ruta_local).exists()
    }
    
    pub fn obtener_ruta_base(&self) -> &Path {
        &self.base_path
    }
}

#[derive(Debug, Clone)]
pub struct ArchivoGuardado {
    pub ruta_local: String,
    pub hash: String,
    pub tamaño_bytes: u64,
    pub nombre_archivo_original: String,
    pub nombre_archivo_hash: String,
}

impl ArchivoGuardado {
    pub fn ruta_relativa(&self) -> String {
        let ruta = PathBuf::from(&self.ruta_local);
        ruta.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or(&self.nombre_archivo_hash)
            .to_string()
    }
}
