use sqlx::SqlitePool;
// Solo necesitamos AppResult y AppError del shared
use escuela_shared::{AppResult, AppError};
// chrono se usa indirectamente vía sqlx para las fechas
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct SearchCriteria {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cedula: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apellido: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nombre: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categoria_documento: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estado: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foliado: Option<bool>,
    #[serde(default = "default_page")]
    pub page: u32,
    #[serde(default = "default_page_size")]
    pub page_size: u32,
}

fn default_page() -> u32 {
    1
}

fn default_page_size() -> u32 {
    20
}

#[derive(Debug, Clone, Serialize)]
pub struct SearchResult<T> {
    pub data: Vec<T>,
    pub total: u64,
    pub page: u32,
    pub page_size: u32,
    pub total_pages: u32,
}

#[derive(Debug, Clone, Serialize)]
pub struct ExpedienteSearchResult {
    pub id: String,
    pub nombres: String,
    pub apellidos: String,
    pub cedula: String,
    pub email: String,
    pub estado: String,
    pub documentos_count: usize,
    pub documentos_foliados_count: usize,
    pub creado_en: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct DocumentoSearchResult {
    pub id: String,
    pub nombre_archivo: String,
    pub categoria: String,
    pub hash: String,
    pub foliado: bool,
    pub expediente_id: String,
    pub expediente_nombres: String,
    pub expediente_apellidos: String,
    pub creado_en: String,
}

pub struct SearchService {
    pool: SqlitePool,
}

impl SearchService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn buscar_expedientes(
        &self,
        criteria: SearchCriteria,
    ) -> AppResult<SearchResult<ExpedienteSearchResult>> {
        let offset = (criteria.page - 1) * criteria.page_size;
        
        let mut query = String::from(
            "SELECT e.id, e.nombres, e.apellidos, e.cedula, e.email, e.estado, e.creado_en, 
                    COUNT(d.id) as documentos_count,
                    SUM(CASE WHEN d.foliado = 1 THEN 1 ELSE 0 END) as documentos_foliados_count
             FROM expedientes e
             LEFT JOIN documentos d ON e.id = d.expediente_id
             WHERE 1=1"
        );
        
        let mut params: Vec<String> = Vec::new();
        let mut param_count = 0;
        
        if let Some(ref cedula) = criteria.cedula {
            param_count += 1;
            query.push_str(&format!(" AND e.cedula LIKE ?{}", param_count));
            params.push(format!("%{}%", cedula));
        }
        
        if let Some(ref apellido) = criteria.apellido {
            param_count += 1;
            query.push_str(&format!(" AND e.apellidos LIKE ?{}", param_count));
            params.push(format!("%{}%", apellido));
        }
        
        if let Some(ref nombre) = criteria.nombre {
            param_count += 1;
            query.push_str(&format!(" AND e.nombres LIKE ?{}", param_count));
            params.push(format!("%{}%", nombre));
        }
        
        if let Some(ref estado) = criteria.estado {
            param_count += 1;
            query.push_str(&format!(" AND e.estado = ?{}", param_count));
            params.push(estado.clone());
        }
        
        if let Some(foliado) = criteria.foliado {
            if foliado {
                query.push_str(" AND EXISTS (SELECT 1 FROM documentos d2 WHERE d2.expediente_id = e.id AND d2.foliado = 1)");
            } else {
                query.push_str(" AND NOT EXISTS (SELECT 1 FROM documentos d2 WHERE d2.expediente_id = e.id AND d2.foliado = 1)");
            }
        }
        
        query.push_str(" GROUP BY e.id, e.nombres, e.apellidos, e.cedula, e.email, e.estado, e.creado_en");
        query.push_str(" ORDER BY e.creado_en DESC");
        query.push_str(&format!(" LIMIT ?{} OFFSET ?{}", param_count + 1, param_count + 2));
        
        let total_query = query
            .replace("SELECT e.id, e.nombres, e.apellidos, e.cedula, e.email, e.estado, e.creado_en,", "SELECT COUNT(*) as total,")
            .replace("COUNT(d.id) as documentos_count,", "")
            .replace("SUM(CASE WHEN d.foliado = 1 THEN 1 ELSE 0 END) as documentos_foliados_count", "")
            .replace(&format!(" LIMIT ?{} OFFSET ?{}", param_count + 1, param_count + 2), "");
        
        let mut query_builder = sqlx::query_as::<_, ExpedienteSearchRow>(&query);
        for param in &params {
            query_builder = query_builder.bind(param);
        }
        query_builder = query_builder.bind(criteria.page_size as i64);
        query_builder = query_builder.bind(offset as i64);
        
        let rows = query_builder
            .fetch_all(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        
        let mut total_query_builder = sqlx::query_as::<_, TotalRow>(&total_query);
        for param in &params {
            total_query_builder = total_query_builder.bind(param);
        }
        
        let total_row = total_query_builder
            .fetch_one(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        
        let data: Vec<ExpedienteSearchResult> = rows
            .into_iter()
            .map(|row| ExpedienteSearchResult {
                id: row.id,
                nombres: row.nombres,
                apellidos: row.apellidos,
                cedula: row.cedula,
                email: row.email,
                estado: row.estado,
                documentos_count: row.documentos_count as usize,
                documentos_foliados_count: row.documentos_foliados_count as usize,
                creado_en: row.creado_en,
            })
            .collect();
        
        let total_pages = ((total_row.total as f64) / (criteria.page_size as f64)).ceil() as u32;
        
        Ok(SearchResult {
            data,
            total: total_row.total as u64,
            page: criteria.page,
            page_size: criteria.page_size,
            total_pages,
        })
    }

    pub async fn buscar_documentos(
        &self,
        criteria: SearchCriteria,
    ) -> AppResult<SearchResult<DocumentoSearchResult>> {
        let offset = (criteria.page - 1) * criteria.page_size;
        
        let mut query = String::from(
            "SELECT d.id, d.nombre_archivo, d.categoria, d.hash, d.foliado, d.expediente_id, 
                    d.creado_en, e.nombres as expediente_nombres, e.apellidos as expediente_apellidos
             FROM documentos d
             INNER JOIN expedientes e ON d.expediente_id = e.id
             WHERE 1=1"
        );
        
        let mut params: Vec<String> = Vec::new();
        let mut param_count = 0;
        
        if let Some(ref cedula) = criteria.cedula {
            param_count += 1;
            query.push_str(&format!(" AND e.cedula LIKE ?{}", param_count));
            params.push(format!("%{}%", cedula));
        }
        
        if let Some(ref apellido) = criteria.apellido {
            param_count += 1;
            query.push_str(&format!(" AND e.apellidos LIKE ?{}", param_count));
            params.push(format!("%{}%", apellido));
        }
        
        if let Some(ref nombre) = criteria.nombre {
            param_count += 1;
            query.push_str(&format!(" AND e.nombres LIKE ?{}", param_count));
            params.push(format!("%{}%", nombre));
        }
        
        if let Some(ref categoria) = criteria.categoria_documento {
            param_count += 1;
            query.push_str(&format!(" AND d.categoria = ?{}", param_count));
            params.push(categoria.clone());
        }
        
        if let Some(foliado) = criteria.foliado {
            param_count += 1;
            query.push_str(&format!(" AND d.foliado = ?{}", param_count));
            params.push(if foliado { "1" } else { "0" }.to_string());
        }
        
        if let Some(ref estado) = criteria.estado {
            param_count += 1;
            query.push_str(&format!(" AND e.estado = ?{}", param_count));
            params.push(estado.clone());
        }
        
        query.push_str(" ORDER BY d.creado_en DESC");
        query.push_str(&format!(" LIMIT ?{} OFFSET ?{}", param_count + 1, param_count + 2));
        
        let total_query = query
            .replace("SELECT d.id, d.nombre_archivo, d.categoria, d.hash, d.foliado, d.expediente_id,", "SELECT COUNT(*) as total,")
            .replace("d.creado_en, e.nombres as expediente_nombres, e.apellidos as expediente_apellidos", "")
            .replace(&format!(" LIMIT ?{} OFFSET ?{}", param_count + 1, param_count + 2), "");
        
        let mut query_builder = sqlx::query_as::<_, DocumentoSearchRow>(&query);
        for param in &params {
            query_builder = query_builder.bind(param);
        }
        query_builder = query_builder.bind(criteria.page_size as i64);
        query_builder = query_builder.bind(offset as i64);
        
        let rows = query_builder
            .fetch_all(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        
        let mut total_query_builder = sqlx::query_as::<_, TotalRow>(&total_query);
        for param in &params {
            total_query_builder = total_query_builder.bind(param);
        }
        
        let total_row = total_query_builder
            .fetch_one(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        
        let data: Vec<DocumentoSearchResult> = rows
            .into_iter()
            .map(|row| DocumentoSearchResult {
                id: row.id,
                nombre_archivo: row.nombre_archivo,
                categoria: row.categoria,
                hash: row.hash,
                foliado: row.foliado == 1,
                expediente_id: row.expediente_id,
                expediente_nombres: row.expediente_nombres,
                expediente_apellidos: row.expediente_apellidos,
                creado_en: row.creado_en,
            })
            .collect();
        
        let total_pages = ((total_row.total as f64) / (criteria.page_size as f64)).ceil() as u32;
        
        Ok(SearchResult {
            data,
            total: total_row.total as u64,
            page: criteria.page,
            page_size: criteria.page_size,
            total_pages,
        })
    }

    pub async fn buscar_por_termino_general(
        &self,
        termino: &str,
        page: u32,
        page_size: u32,
    ) -> AppResult<SearchResult<ExpedienteSearchResult>> {
        let offset = (page - 1) * page_size;
        let pattern = format!("%{}%", termino);
        
        let query = r#"
            SELECT e.id, e.nombres, e.apellidos, e.cedula, e.email, e.estado, e.creado_en, 
                   COUNT(d.id) as documentos_count,
                   SUM(CASE WHEN d.foliado = 1 THEN 1 ELSE 0 END) as documentos_foliados_count
            FROM expedientes e
            LEFT JOIN documentos d ON e.id = d.expediente_id
            WHERE e.nombres LIKE ?1 
               OR e.apellidos LIKE ?1 
               OR e.cedula LIKE ?1 
               OR e.email LIKE ?1
            GROUP BY e.id, e.nombres, e.apellidos, e.cedula, e.email, e.estado, e.creado_en
            ORDER BY 
                CASE 
                    WHEN e.cedula LIKE ?1 THEN 1
                    WHEN e.apellidos LIKE ?1 THEN 2
                    WHEN e.nombres LIKE ?1 THEN 3
                    ELSE 4
                END,
                e.creado_en DESC
            LIMIT ?2 OFFSET ?3
        "#;
        
        let total_query = r#"
            SELECT COUNT(*) as total
            FROM expedientes e
            WHERE e.nombres LIKE ?1 
               OR e.apellidos LIKE ?1 
               OR e.cedula LIKE ?1 
               OR e.email LIKE ?1
        "#;
        
        let rows = sqlx::query_as::<_, ExpedienteSearchRow>(query)
            .bind(&pattern)
            .bind(page_size as i64)
            .bind(offset as i64)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        
        let total_row = sqlx::query_as::<_, TotalRow>(total_query)
            .bind(&pattern)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        
        let data: Vec<ExpedienteSearchResult> = rows
            .into_iter()
            .map(|row| ExpedienteSearchResult {
                id: row.id,
                nombres: row.nombres,
                apellidos: row.apellidos,
                cedula: row.cedula,
                email: row.email,
                estado: row.estado,
                documentos_count: row.documentos_count as usize,
                documentos_foliados_count: row.documentos_foliados_count as usize,
                creado_en: row.creado_en,
            })
            .collect();
        
        let total_pages = ((total_row.total as f64) / (page_size as f64)).ceil() as u32;
        
        Ok(SearchResult {
            data,
            total: total_row.total as u64,
            page,
            page_size,
            total_pages,
        })
    }
}

#[derive(sqlx::FromRow)]
struct ExpedienteSearchRow {
    id: String,
    nombres: String,
    apellidos: String,
    cedula: String,
    email: String,
    estado: String,
    creado_en: String,
    documentos_count: i64,
    documentos_foliados_count: i64,
}

#[derive(sqlx::FromRow)]
struct DocumentoSearchRow {
    id: String,
    nombre_archivo: String,
    categoria: String,
    hash: String,
    foliado: i32,
    expediente_id: String,
    creado_en: String,
    expediente_nombres: String,
    expediente_apellidos: String,
}

#[derive(sqlx::FromRow)]
struct TotalRow {
    total: i64,
}
