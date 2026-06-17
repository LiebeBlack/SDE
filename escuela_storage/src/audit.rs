use sqlx::SqlitePool;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use escuela_shared::{AppResult, AppError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccionAuditoria {
    ConsultaExpediente,
    CreacionExpediente,
    ModificacionExpediente,
    EliminacionExpediente,
    ConsultaDocumento,
    SubidaDocumento,
    ModificacionDocumento,
    EliminacionDocumento,
    FoliadoDocumento,
    BusquedaAvanzada,
    LoginUsuario,
    LogoutUsuario,
    CambioEstadoExpediente,
}

impl AccionAuditoria {
    pub fn as_str(&self) -> &'static str {
        match self {
            AccionAuditoria::ConsultaExpediente => "CONSULTA_EXPEDIENTE",
            AccionAuditoria::CreacionExpediente => "CREACION_EXPEDIENTE",
            AccionAuditoria::ModificacionExpediente => "MODIFICACION_EXPEDIENTE",
            AccionAuditoria::EliminacionExpediente => "ELIMINACION_EXPEDIENTE",
            AccionAuditoria::ConsultaDocumento => "CONSULTA_DOCUMENTO",
            AccionAuditoria::SubidaDocumento => "SUBIDA_DOCUMENTO",
            AccionAuditoria::ModificacionDocumento => "MODIFICACION_DOCUMENTO",
            AccionAuditoria::EliminacionDocumento => "ELIMINACION_DOCUMENTO",
            AccionAuditoria::FoliadoDocumento => "FOLIADO_DOCUMENTO",
            AccionAuditoria::BusquedaAvanzada => "BUSQUEDA_AVANZADA",
            AccionAuditoria::LoginUsuario => "LOGIN_USUARIO",
            AccionAuditoria::LogoutUsuario => "LOGOUT_USUARIO",
            AccionAuditoria::CambioEstadoExpediente => "CAMBIO_ESTADO_EXPEDIENTE",
        }
    }

    pub fn from_str(s: &str) -> AppResult<Self> {
        match s {
            "CONSULTA_EXPEDIENTE" => Ok(AccionAuditoria::ConsultaExpediente),
            "CREACION_EXPEDIENTE" => Ok(AccionAuditoria::CreacionExpediente),
            "MODIFICACION_EXPEDIENTE" => Ok(AccionAuditoria::ModificacionExpediente),
            "ELIMINACION_EXPEDIENTE" => Ok(AccionAuditoria::EliminacionExpediente),
            "CONSULTA_DOCUMENTO" => Ok(AccionAuditoria::ConsultaDocumento),
            "SUBIDA_DOCUMENTO" => Ok(AccionAuditoria::SubidaDocumento),
            "MODIFICACION_DOCUMENTO" => Ok(AccionAuditoria::ModificacionDocumento),
            "ELIMINACION_DOCUMENTO" => Ok(AccionAuditoria::EliminacionDocumento),
            "FOLIADO_DOCUMENTO" => Ok(AccionAuditoria::FoliadoDocumento),
            "BUSQUEDA_AVANZADA" => Ok(AccionAuditoria::BusquedaAvanzada),
            "LOGIN_USUARIO" => Ok(AccionAuditoria::LoginUsuario),
            "LOGOUT_USUARIO" => Ok(AccionAuditoria::LogoutUsuario),
            "CAMBIO_ESTADO_EXPEDIENTE" => Ok(AccionAuditoria::CambioEstadoExpediente),
            _ => Err(AppError::ValidationError(format!("Acción de auditoría inválida: {}", s))),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistroAuditoria {
    pub id: String,
    pub usuario_id: Option<String>,
    pub accion: AccionAuditoria,
    pub timestamp: DateTime<Utc>,
    pub detalles: String,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
}

pub struct AuditService {
    pool: SqlitePool,
}

impl AuditService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn registrar_accion(
        &self,
        usuario_id: Option<String>,
        accion: AccionAuditoria,
        detalles: String,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> AppResult<()> {
        let id = Uuid::new_v4().to_string();
        let timestamp = Utc::now();
        let accion_str = accion.as_str();

        sqlx::query(
            r#"
            INSERT INTO auditoria_accesos (id, usuario_id, accion, timestamp, detalles, ip_address, user_agent)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&id)
        .bind(&usuario_id)
        .bind(accion_str)
        .bind(timestamp.to_rfc3339())
        .bind(&detalles)
        .bind(&ip_address)
        .bind(&user_agent)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    pub async fn obtener_historial_usuario(
        &self,
        usuario_id: &str,
        limite: u32,
    ) -> AppResult<Vec<RegistroAuditoria>> {
        let rows = sqlx::query_as::<_, AuditoriaRow>(
            r#"
            SELECT id, usuario_id, accion, timestamp, detalles, ip_address, user_agent
            FROM auditoria_accesos
            WHERE usuario_id = ?
            ORDER BY timestamp DESC
            LIMIT ?
            "#,
        )
        .bind(usuario_id)
        .bind(limite as i64)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        rows.into_iter()
            .map(|row| row.to_registro_auditoria())
            .collect()
    }

    pub async fn obtener_historial_accion(
        &self,
        accion: AccionAuditoria,
        limite: u32,
    ) -> AppResult<Vec<RegistroAuditoria>> {
        let rows = sqlx::query_as::<_, AuditoriaRow>(
            r#"
            SELECT id, usuario_id, accion, timestamp, detalles, ip_address, user_agent
            FROM auditoria_accesos
            WHERE accion = ?
            ORDER BY timestamp DESC
            LIMIT ?
            "#,
        )
        .bind(accion.as_str())
        .bind(limite as i64)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        rows.into_iter()
            .map(|row| row.to_registro_auditoria())
            .collect()
    }

    pub async fn obtener_historial_completo(
        &self,
        limite: u32,
        offset: u32,
    ) -> AppResult<Vec<RegistroAuditoria>> {
        let rows = sqlx::query_as::<_, AuditoriaRow>(
            r#"
            SELECT id, usuario_id, accion, timestamp, detalles, ip_address, user_agent
            FROM auditoria_accesos
            ORDER BY timestamp DESC
            LIMIT ? OFFSET ?
            "#,
        )
        .bind(limite as i64)
        .bind(offset as i64)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        rows.into_iter()
            .map(|row| row.to_registro_auditoria())
            .collect()
    }

    pub async fn contar_registros_totales(&self) -> AppResult<u64> {
        let row = sqlx::query_as::<_, CountRow>(
            "SELECT COUNT(*) as total FROM auditoria_accesos"
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(row.total as u64)
    }

    pub async fn obtener_estadisticas(&self) -> AppResult<AuditoriaEstadisticas> {
        let total_registros = self.contar_registros_totales().await?;

        let acciones_por_tipo = sqlx::query_as::<_, AccionCountRow>(
            r#"
            SELECT accion, COUNT(*) as count
            FROM auditoria_accesos
            GROUP BY accion
            ORDER BY count DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(AuditoriaEstadisticas {
            total_registros,
            acciones_por_tipo: acciones_por_tipo
                .into_iter()
                .map(|row| (row.accion, row.count as u64))
                .collect(),
        })
    }
}

#[derive(sqlx::FromRow)]
struct AuditoriaRow {
    id: String,
    usuario_id: Option<String>,
    accion: String,
    timestamp: String,
    detalles: String,
    ip_address: Option<String>,
    user_agent: Option<String>,
}

impl AuditoriaRow {
    fn to_registro_auditoria(self) -> AppResult<RegistroAuditoria> {
        Ok(RegistroAuditoria {
            id: self.id,
            usuario_id: self.usuario_id,
            accion: AccionAuditoria::from_str(&self.accion)?,
            timestamp: DateTime::parse_from_rfc3339(&self.timestamp)
                .map_err(|e| AppError::InternalError(e.to_string()))?
                .with_timezone(&Utc),
            detalles: self.detalles,
            ip_address: self.ip_address,
            user_agent: self.user_agent,
        })
    }
}

#[derive(sqlx::FromRow)]
struct CountRow {
    total: i64,
}

#[derive(sqlx::FromRow)]
struct AccionCountRow {
    accion: String,
    count: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct AuditoriaEstadisticas {
    pub total_registros: u64,
    pub acciones_por_tipo: Vec<(String, u64)>,
}
