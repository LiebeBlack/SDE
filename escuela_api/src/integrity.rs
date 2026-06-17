use std::path::Path;
use std::fs;
use escuela_storage::Database;
use anyhow::Result;

#[derive(Debug)]
pub struct IntegrityReport {
    pub database_valid: bool,
    pub database_path: String,
    pub expedientes_count: u64,
    pub documentos_count: u64,
    pub storage_valid: bool,
    pub storage_path: String,
    pub storage_folders_count: usize,
    pub storage_files_count: usize,
    pub audit_records_count: u64,
    pub issues: Vec<String>,
}

impl IntegrityReport {
    pub fn is_healthy(&self) -> bool {
        self.database_valid && self.storage_valid && self.issues.is_empty()
    }
}

pub async fn verify_integrity(database_path: &str, storage_path: &str) -> Result<IntegrityReport> {
    let mut issues = Vec::new();
    let mut report = IntegrityReport {
        database_valid: false,
        database_path: database_path.to_string(),
        expedientes_count: 0,
        documentos_count: 0,
        storage_valid: false,
        storage_path: storage_path.to_string(),
        storage_folders_count: 0,
        storage_files_count: 0,
        audit_records_count: 0,
        issues: Vec::new(),
    };

    println!("🔍 Iniciando verificación de integridad del sistema...");

    // Verificar base de datos
    let db_path = Path::new(database_path);
    if !db_path.exists() {
        issues.push(format!("Base de datos no encontrada: {}", database_path));
    } else {
        let metadata = fs::metadata(db_path)?;
        if metadata.len() == 0 {
            issues.push("Base de datos está vacía (0 bytes)".to_string());
        } else {
            report.database_valid = true;
            println!("✅ Base de datos encontrada: {} bytes", metadata.len());
        }
    }

    // Si la base de datos existe, conectar y contar registros
    if report.database_valid {
        match Database::new(database_path).await {
            Ok(database) => {
                let pool = database.pool();
                
                // Contar expedientes
                match sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM expedientes")
                    .fetch_one(pool)
                    .await
                {
                    Ok(count) => {
                        report.expedientes_count = count as u64;
                        println!("📊 Expedientes registrados: {}", report.expedientes_count);
                    }
                    Err(e) => {
                        issues.push(format!("Error al contar expedientes: {}", e));
                    }
                }

                // Contar documentos
                match sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM documentos")
                    .fetch_one(pool)
                    .await
                {
                    Ok(count) => {
                        report.documentos_count = count as u64;
                        println!("📄 Documentos almacenados: {}", report.documentos_count);
                    }
                    Err(e) => {
                        issues.push(format!("Error al contar documentos: {}", e));
                    }
                }

                // Contar registros de auditoría
                match sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM auditoria_accesos")
                    .fetch_one(pool)
                    .await
                {
                    Ok(count) => {
                        report.audit_records_count = count as u64;
                        println!("📝 Registros de auditoría: {}", report.audit_records_count);
                    }
                    Err(e) => {
                        issues.push(format!("Error al contar registros de auditoría: {}", e));
                    }
                }

                database.close().await?;
            }
            Err(e) => {
                issues.push(format!("Error al conectar a base de datos: {}", e));
                report.database_valid = false;
            }
        }
    }

    // Verificar almacenamiento de archivos
    let storage = Path::new(storage_path);
    if !storage.exists() {
        issues.push(format!("Directorio de almacenamiento no encontrado: {}", storage_path));
        // Intentar crear el directorio
        if let Err(e) = fs::create_dir_all(storage) {
            issues.push(format!("Error al crear directorio de almacenamiento: {}", e));
        } else {
            println!("📁 Directorio de almacenamiento creado: {}", storage_path);
            report.storage_valid = true;
        }
    } else {
        report.storage_valid = true;
        println!("✅ Directorio de almacenamiento encontrado: {}", storage_path);

        // Contar carpetas de expedientes
        if let Ok(entries) = fs::read_dir(storage) {
            let mut folders = 0;
            let mut files = 0;
            
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    folders += 1;
                    // Verificar si la carpeta tiene archivos
                    if let Ok(sub_entries) = fs::read_dir(&path) {
                        for sub_entry in sub_entries.flatten() {
                            if sub_entry.path().is_file() {
                                files += 1;
                            }
                        }
                    }
                }
            }
            
            report.storage_folders_count = folders;
            report.storage_files_count = files;
            println!("📂 Carpetas de expedientes: {}", folders);
            println!("📁 Archivos almacenados: {}", files);

            // Verificar consistencia entre base de datos y archivos
            if report.expedientes_count > 0 && folders as u64 != report.expedientes_count {
                issues.push(format!(
                    "Inconsistencia detectada: {} expedientes en DB vs {} carpetas en almacenamiento",
                    report.expedientes_count, folders
                ));
            }

            if report.documentos_count > 0 && files as u64 != report.documentos_count {
                issues.push(format!(
                    "Inconsistencia detectada: {} documentos en DB vs {} archivos en almacenamiento",
                    report.documentos_count, files
                ));
            }
        }
    }

    // Reporte final
    println!("\n📋 REPORTE DE INTEGRIDAD DEL SISTEMA");
    println!("═══════════════════════════════════════");
    println!("Base de datos: {}", if report.database_valid { "✅ VÁLIDA" } else { "❌ INVÁLIDA" });
    println!("Almacenamiento: {}", if report.storage_valid { "✅ VÁLIDO" } else { "❌ INVÁLIDO" });
    println!("Expedientes: {}", report.expedientes_count);
    println!("Documentos: {}", report.documentos_count);
    println!("Registros auditoría: {}", report.audit_records_count);
    println!("Carpetas almacenamiento: {}", report.storage_folders_count);
    println!("Archivos almacenamiento: {}", report.storage_files_count);
    
    if !issues.is_empty() {
        println!("\n⚠️  PROBLEMAS DETECTADOS:");
        for (i, issue) in issues.iter().enumerate() {
            println!("  {}. {}", i + 1, issue);
        }
    } else {
        println!("\n✅ Sistema en estado óptimo - Sin problemas detectados");
    }
    println!("═══════════════════════════════════════\n");

    report.issues = issues;
    Ok(report)
}

pub fn print_startup_banner() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║                                                              ║");
    println!("║          🏫 SISTEMA DE GESTIÓN ESCOLAR - TESIS              ║");
    println!("║                                                              ║");
    println!("║          Arquitectura Modular en Rust                        ║");
    println!("║          Gestión Documental Integral                         ║");
    println!("║          Seguridad y Auditoría                               ║");
    println!("║                                                              ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();
}
