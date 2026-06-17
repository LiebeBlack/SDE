use anyhow::Result;
use tracing::{info, error};
use std::path::Path;
use std::fs::{self, File};
use std::io::{Read, Write};
use zip::write::SimpleFileOptions;
use chrono::Local;

pub struct BackupService {
    database_path: String,
    storage_path: String,
    backups_dir: String,
    max_backups: usize,
}

impl BackupService {
    pub fn new(database_path: &str, storage_path: &str) -> Result<Self> {
        let backups_dir = "backups".to_string();
        if !Path::new(&backups_dir).exists() {
            fs::create_dir_all(&backups_dir)?;
        }
        
        Ok(Self {
            database_path: database_path.to_string(),
            storage_path: storage_path.to_string(),
            backups_dir,
            max_backups: 6,
        })
    }

    pub async fn create_backup(&self) -> Result<String> {
        let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
        let backup_filename = format!("backup_{}.zip", timestamp);
        let backup_path = Path::new(&self.backups_dir).join(&backup_filename);

        info!("Iniciando copia de seguridad: {}", backup_path.display());

        let file = File::create(&backup_path)?;
        let mut zip = zip::ZipWriter::new(file);
        let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);

        // 1. Respaldar base de datos
        let db_path = Path::new(&self.database_path);
        if db_path.exists() {
            info!("Agregando base de datos al respaldo...");
            zip.start_file(db_path.file_name().unwrap().to_string_lossy(), options)?;
            let mut f = File::open(db_path)?;
            let mut buffer = Vec::new();
            f.read_to_end(&mut buffer)?;
            zip.write_all(&buffer)?;
        } else {
            error!("Base de datos no encontrada para respaldo en {}", self.database_path);
        }

        // 2. Respaldar archivos de storage
        let storage_path = Path::new(&self.storage_path);
        if storage_path.exists() {
            info!("Agregando archivos de almacenamiento al respaldo...");
            self.add_dir_to_zip(&mut zip, storage_path, storage_path, options)?;
        }

        zip.finish()?;
        info!("Copia de seguridad completada con éxito: {}", backup_path.display());

        // 3. Limpiar respaldos antiguos
        self.cleanup_old_backups()?;

        Ok(backup_path.to_string_lossy().to_string())
    }

    fn add_dir_to_zip(
        &self,
        zip: &mut zip::ZipWriter<File>,
        dir: &Path,
        base_dir: &Path,
        options: SimpleFileOptions,
    ) -> Result<()> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            
            let relative_path = path.strip_prefix(base_dir)
                .unwrap_or(&path)
                .to_string_lossy()
                .into_owned();

            let zip_path = format!("{}/{}", base_dir.file_name().unwrap().to_string_lossy(), relative_path);

            if path.is_dir() {
                zip.add_directory(zip_path, options)?;
                self.add_dir_to_zip(zip, &path, base_dir, options)?;
            } else {
                zip.start_file(zip_path, options)?;
                let mut f = File::open(&path)?;
                let mut buffer = Vec::new();
                f.read_to_end(&mut buffer)?;
                zip.write_all(&buffer)?;
            }
        }
        Ok(())
    }

    fn cleanup_old_backups(&self) -> Result<()> {
        let mut backups = Vec::new();
        
        for entry in fs::read_dir(&self.backups_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() && path.extension().map_or(false, |ext| ext == "zip") {
                if let Ok(metadata) = entry.metadata() {
                    if let Ok(modified) = metadata.modified() {
                        backups.push((path, modified));
                    }
                }
            }
        }

        // Ordenar por fecha de modificación (más reciente primero)
        backups.sort_by(|a, b| b.1.cmp(&a.1));

        // Si hay más de max_backups, eliminar los más antiguos
        if backups.len() > self.max_backups {
            info!("Limpiando respaldos antiguos. Manteniendo los últimos {}.", self.max_backups);
            for (path, _) in backups.into_iter().skip(self.max_backups) {
                if let Err(e) = fs::remove_file(&path) {
                    error!("Error al eliminar respaldo antiguo {}: {}", path.display(), e);
                } else {
                    info!("Respaldo antiguo eliminado: {}", path.display());
                }
            }
        }

        Ok(())
    }
}
