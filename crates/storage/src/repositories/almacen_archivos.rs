use app_core::services::documento_service::AlmacenArchivos;
use std::path::{Path, PathBuf};
use uuid::Uuid;

/// Extensiones de archivos permitidas para almacenamiento
const EXTENSIONES_PERMITIDAS: &[&str] = &["pdf", "png", "jpg", "jpeg", "gif", "bmp", "webp"];

/// Tamaño máximo de archivo en bytes (50MB)
const TAMANO_MAXIMO_BYTES: u64 = 50 * 1024 * 1024;

/// Copia archivos (PDF/imagen) a data/documentos/<uuid>.<ext>, dejando
/// intacto el original que el usuario seleccionó.
pub struct AlmacenLocal {
    directorio_base: PathBuf,
}

impl AlmacenLocal {
    pub fn new(directorio_base: impl Into<PathBuf>) -> Self {
        Self {
            directorio_base: directorio_base.into(),
        }
    }

    /// Valida que la extensión del archivo esté permitida
    fn validar_extension(&self, ruta: &Path) -> anyhow::Result<()> {
        let ext = ruta
            .extension()
            .and_then(|e| e.to_str())
            .ok_or_else(|| anyhow::anyhow!("El archivo no tiene extensión"))?;

        let ext_lower = ext.to_lowercase();
        
        if !EXTENSIONES_PERMITIDAS.contains(&ext_lower.as_str()) {
            return Err(anyhow::anyhow!(
                "Extensión no permitida: {}. Extensiones permitidas: {}",
                ext,
                EXTENSIONES_PERMITIDAS.join(", ")
            ));
        }
        
        Ok(())
    }

    /// Valida que el archivo exista y sea accesible
    fn validar_archivo_existe(&self, ruta: &Path) -> anyhow::Result<()> {
        if !ruta.exists() {
            return Err(anyhow::anyhow!("El archivo no existe: {}", ruta.display()));
        }
        
        if !ruta.is_file() {
            return Err(anyhow::anyhow!("La ruta no es un archivo: {}", ruta.display()));
        }
        
        Ok(())
    }

    /// Valida el tamaño del archivo
    fn validar_tamano(&self, ruta: &Path) -> anyhow::Result<u64> {
        let metadata = std::fs::metadata(ruta)
            .map_err(|e| anyhow::anyhow!("Error al obtener metadatos del archivo: {}", e))?;
        
        let tamano = metadata.len();
        
        if tamano == 0 {
            return Err(anyhow::anyhow!("El archivo está vacío"));
        }
        
        if tamano > TAMANO_MAXIMO_BYTES {
            return Err(anyhow::anyhow!(
                "El archivo excede el tamaño máximo permitido de {}MB (actual: {}MB)",
                TAMANO_MAXIMO_BYTES / 1024 / 1024,
                tamano / 1024 / 1024
            ));
        }
        
        Ok(tamano)
    }

    /// Valida que el nombre del archivo sea seguro (sin caracteres peligrosos)
    fn validar_nombre_seguro(&self, ruta: &Path) -> anyhow::Result<()> {
        let nombre = ruta
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| anyhow::anyhow!("Nombre de archivo inválido"))?;

        // Caracteres peligrosos que no deben estar en nombres de archivo
        let caracteres_peligrosos = ['<', '>', ':', '"', '|', '?', '*', '\0', '\n', '\r'];
        
        for c in nombre.chars() {
            if caracteres_peligrosos.contains(&c) {
                return Err(anyhow::anyhow!("El nombre del archivo contiene caracteres inválidos"));
            }
        }
        
        Ok(())
    }

    /// Elimina un archivo del almacenamiento
    pub fn eliminar_archivo(&self, ruta_relativa: &str) -> anyhow::Result<()> {
        let ruta_completa = self.directorio_base.join(ruta_relativa);
        
        if ruta_completa.exists() {
            std::fs::remove_file(&ruta_completa)
                .map_err(|e| anyhow::anyhow!("Error al eliminar archivo: {}", e))?;
        }
        
        Ok(())
    }

    /// Obtiene la ruta completa de un archivo almacenado
    pub fn obtener_ruta_completa(&self, ruta_relativa: &str) -> PathBuf {
        self.directorio_base.join(ruta_relativa)
    }
}

impl AlmacenArchivos for AlmacenLocal {
    fn guardar_archivo(&self, origen: &Path, id: Uuid) -> anyhow::Result<String> {
        // Validaciones antes de copiar
        self.validar_archivo_existe(origen)?;
        self.validar_nombre_seguro(origen)?;
        self.validar_extension(origen)?;
        let tamano = self.validar_tamano(origen)?;

        // Crear directorio si no existe
        std::fs::create_dir_all(&self.directorio_base)
            .map_err(|e| anyhow::anyhow!("Error al crear directorio de almacenamiento: {}", e))?;

        // Obtener extensión y construir nombre destino
        let ext = origen
            .extension()
            .map(|e| e.to_string_lossy().to_lowercase())
            .unwrap_or_default();
        
        let nombre_destino = if ext.is_empty() {
            id.to_string()
        } else {
            format!("{}.{}", id, ext)
        };
        
        let destino = self.directorio_base.join(&nombre_destino);

        // Verificar si ya existe el archivo destino
        if destino.exists() {
            return Err(anyhow::anyhow!(
                "Ya existe un archivo con el mismo ID en el almacenamiento"
            ));
        }

        // Copiar archivo
        std::fs::copy(origen, &destino)
            .map_err(|e| anyhow::anyhow!("Error al copiar archivo: {}", e))?;

        // Verificar que la copia fue exitosa
        if !destino.exists() {
            return Err(anyhow::anyhow!("La copia del archivo no se completó correctamente"));
        }

        let metadata_destino = std::fs::metadata(&destino)?;
        if metadata_destino.len() != tamano {
            return Err(anyhow::anyhow!("El archivo copiado tiene un tamaño diferente al original"));
        }

        Ok(destino.to_string_lossy().to_string())
    }
}
