use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Estado laboral del empleado
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EstadoEmpleado {
    Activo,
    Baja,
    Vacaciones,
    LicenciaMedica,
    Suspendido,
}

impl EstadoEmpleado {
    /// Valida si el estado permite registrar asistencia
    pub fn permite_asistencia(&self) -> bool {
        matches!(self, EstadoEmpleado::Activo)
    }
}

impl std::fmt::Display for EstadoEmpleado {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            EstadoEmpleado::Activo => "Activo",
            EstadoEmpleado::Baja => "Baja",
            EstadoEmpleado::Vacaciones => "Vacaciones",
            EstadoEmpleado::LicenciaMedica => "Licencia Médica",
            EstadoEmpleado::Suspendido => "Suspendido",
        };
        write!(f, "{s}")
    }
}

/// Tipo de contrato laboral
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TipoContrato {
    TiempoCompleto,
    MedioTiempo,
    PorHoras,
    Temporal,
    Practicas,
    ContratoObra,
}

impl TipoContrato {
    /// Obtiene las horas semanales estándar según el tipo de contrato
    pub fn horas_semanales_estandar(&self) -> u32 {
        match self {
            TipoContrato::TiempoCompleto => 40,
            TipoContrato::MedioTiempo => 20,
            TipoContrato::PorHoras => 0, // Variable
            TipoContrato::Temporal => 40,
            TipoContrato::Practicas => 35,
            TipoContrato::ContratoObra => 44,
        }
    }
}

impl std::fmt::Display for TipoContrato {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            TipoContrato::TiempoCompleto => "Tiempo Completo",
            TipoContrato::MedioTiempo => "Medio Tiempo",
            TipoContrato::PorHoras => "Por Horas",
            TipoContrato::Temporal => "Temporal",
            TipoContrato::Practicas => "Prácticas",
            TipoContrato::ContratoObra => "Contrato por Obra",
        };
        write!(f, "{s}")
    }
}

/// Tipo de registro de asistencia
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TipoAsistencia {
    Entrada,
    Salida,
    EntradaAlmuerzo,
    SalidaAlmuerzo,
}

impl std::fmt::Display for TipoAsistencia {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            TipoAsistencia::Entrada => "Entrada",
            TipoAsistencia::Salida => "Salida",
            TipoAsistencia::EntradaAlmuerzo => "Inicio Almuerzo",
            TipoAsistencia::SalidaAlmuerzo => "Fin Almuerzo",
        };
        write!(f, "{s}")
    }
}

/// Registro individual de asistencia
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistroAsistencia {
    pub id: Uuid,
    pub empleado_id: Uuid,
    pub fecha: NaiveDate,
    pub hora: NaiveTime,
    pub tipo: TipoAsistencia,
    pub notas: Option<String>,
    pub creado_en: DateTime<Utc>,
}

impl RegistroAsistencia {
    pub fn nuevo(
        empleado_id: Uuid,
        fecha: NaiveDate,
        hora: NaiveTime,
        tipo: TipoAsistencia,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            empleado_id,
            fecha,
            hora,
            tipo,
            notas: None,
            creado_en: Utc::now(),
        }
    }

    /// Valida que el registro sea lógico (ej: no tener salida sin entrada previa)
    pub fn es_valido_con_contexto(&self, registros_previos: &[RegistroAsistencia]) -> bool {
        match self.tipo {
            TipoAsistencia::Entrada => {
                // No puede tener entrada sin haber salido del día anterior
                let ultimo_registro = registros_previos
                    .iter()
                    .filter(|r| r.fecha == self.fecha)
                    .last();
                
                match ultimo_registro {
                    Some(ultimo) => {
                        // Si el último fue entrada, no puede tener otra entrada
                        !matches!(ultimo.tipo, TipoAsistencia::Entrada | TipoAsistencia::EntradaAlmuerzo)
                    }
                    None => true, // Primer registro del día, válido
                }
            }
            TipoAsistencia::Salida => {
                // Debe tener una entrada previa el mismo día
                registros_previos
                    .iter()
                    .any(|r| r.fecha == self.fecha && matches!(r.tipo, TipoAsistencia::Entrada | TipoAsistencia::SalidaAlmuerzo))
            }
            TipoAsistencia::EntradaAlmuerzo => {
                // Debe tener entrada previa y no estar ya en almuerzo
                let tiene_entrada = registros_previos
                    .iter()
                    .any(|r| r.fecha == self.fecha && matches!(r.tipo, TipoAsistencia::Entrada));
                let no_en_almuerzo = !registros_previos
                    .iter()
                    .filter(|r| r.fecha == self.fecha)
                    .any(|r| matches!(r.tipo, TipoAsistencia::EntradaAlmuerzo));
                tiene_entrada && no_en_almuerzo
            }
            TipoAsistencia::SalidaAlmuerzo => {
                // Debe tener inicio de almuerzo previo
                registros_previos
                    .iter()
                    .any(|r| r.fecha == self.fecha && matches!(r.tipo, TipoAsistencia::EntradaAlmuerzo))
            }
        }
    }
}

/// Empleado de la institución
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Empleado {
    pub id: Uuid,
    pub cedula: String,
    pub nombre: String,
    pub apellido: String,
    pub email: String,
    pub telefono: Option<String>,
    pub direccion: Option<String>,
    pub cargo: String,
    pub departamento_id: Option<Uuid>,
    pub fecha_contratacion: NaiveDate,
    pub fecha_terminacion: Option<NaiveDate>,
    pub salario: f64,
    pub tipo_contrato: TipoContrato,
    pub estado: EstadoEmpleado,
    pub notas: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Empleado {
    pub fn nuevo(
        cedula: String,
        nombre: String,
        apellido: String,
        email: String,
        cargo: String,
        fecha_contratacion: NaiveDate,
        salario: f64,
        tipo_contrato: TipoContrato,
    ) -> Self {
        let ahora = Utc::now();
        Self {
            id: Uuid::new_v4(),
            cedula,
            nombre,
            apellido,
            email,
            telefono: None,
            direccion: None,
            cargo,
            departamento_id: None,
            fecha_contratacion,
            fecha_terminacion: None,
            salario,
            tipo_contrato,
            estado: EstadoEmpleado::Activo,
            notas: None,
            created_at: ahora,
            updated_at: ahora,
        }
    }

    /// Valida formato de cédula (formato básico sin librerías externas)
    pub fn validar_cedula(cedula: &str) -> bool {
        // Solo dígitos, longitud entre 8 y 15 caracteres
        cedula.chars().all(|c| c.is_ascii_digit()) && (8..=15).contains(&cedula.len())
    }

    /// Valida formato de email (validación básica inline sin librerías externas)
    pub fn validar_email(email: &str) -> bool {
        let email = email.trim().to_lowercase();
        
        // Debe tener exactamente un @
        let arroba_count = email.chars().filter(|&c| c == '@').count();
        if arroba_count != 1 {
            return false;
        }

        let partes: Vec<&str> = email.split('@').collect();
        if partes.len() != 2 {
            return false;
        }

        let local = partes[0];
        let dominio = partes[1];

        // Parte local no puede estar vacía
        if local.is_empty() {
            return false;
        }

        // Dominio debe tener al menos un punto
        if !dominio.contains('.') {
            return false;
        }

        // Dominio debe tener formato válido (ej: ejemplo.com)
        let dominio_partes: Vec<&str> = dominio.split('.').collect();
        if dominio_partes.len() < 2 {
            return false;
        }

        // Cada parte del dominio debe tener al menos 2 caracteres
        for parte in dominio_partes {
            if parte.len() < 2 {
                return false;
            }
        }

        true
    }

    /// Valida que el salario sea positivo y razonable
    pub fn validar_salario(salario: f64) -> bool {
        salario > 0.0 && salario < 1_000_000.0 // Límite arbitrario pero razonable
    }

    /// Calcula la antigüedad en meses
    pub fn antiguedad_meses(&self) -> i64 {
        let hoy = Utc::now().naive_utc().date();
        let contratacion = self.fecha_contratacion;
        
        let meses = (hoy.year() - contratacion.year()) * 12;
        let meses += hoy.month() as i64 - contratacion.month() as i64;
        
        if hoy.day() < contratacion.day() {
            meses - 1
        } else {
            meses
        }
    }

    /// Valida si el empleado está activo y dentro de su periodo contractual
    pub fn es_activo contractualmente(&self) -> bool {
        if self.estado != EstadoEmpleado::Activo {
            return false;
        }

        if let Some(terminacion) = self.fecha_terminacion {
            let hoy = Utc::now().naive_utc().date();
            hoy <= terminacion
        } else {
            true
        }
    }

    pub fn nombre_completo(&self) -> String {
        format!("{} {}", self.nombre, self.apellido)
    }

    /// Valida todos los campos críticos del empleado
    pub fn validar_campos(&self) -> Result<(), String> {
        if !Self::validar_cedula(&self.cedula) {
            return Err("Cédula inválida: debe contener solo dígitos (8-15 caracteres)".to_string());
        }

        if !Self::validar_email(&self.email) {
            return Err("Email inválido: formato incorrecto".to_string());
        }

        if !Self::validar_salario(self.salario) {
            return Err("Salario inválido: debe ser positivo y razonable".to_string());
        }

        if self.nombre.trim().is_empty() {
            return Err("Nombre no puede estar vacío".to_string());
        }

        if self.apellido.trim().is_empty() {
            return Err("Apellido no puede estar vacío".to_string());
        }

        if self.cargo.trim().is_empty() {
            return Err("Cargo no puede estar vacío".to_string());
        }

        Ok(())
    }
}

/// Estructura para calcular horas trabajadas en un periodo
#[derive(Debug, Clone)]
pub struct CalculoHoras {
    pub horas_trabajadas: f64,
    pub horas_extra: f64,
    pub horas_almuerzo: f64,
}

impl CalculoHoras {
    /// Calcula las horas trabajadas basándose en registros de asistencia
    pub fn desde_registros(registros: &[RegistroAsistencia]) -> Self {
        let mut horas_trabajadas = 0.0;
        let mut horas_extra = 0.0;
        let mut horas_almuerzo = 0.0;

        // Agrupar registros por fecha
        let mut registros_por_fecha: std::collections::HashMap<NaiveDate, Vec<&RegistroAsistencia>> = 
            std::collections::HashMap::new();

        for reg in registros {
            registros_por_fecha
                .entry(reg.fecha)
                .or_insert_with(Vec::new)
                .push(reg);
        }

        for (_fecha, regs_dia) in registros_por_fecha {
            let mut entrada: Option<NaiveTime> = None;
            let mut salida: Option<NaiveTime> = None;
            let mut inicio_almuerzo: Option<NaiveTime> = None;
            let mut fin_almuerzo: Option<NaiveTime> = None;

            for reg in regs_dia {
                match reg.tipo {
                    TipoAsistencia::Entrada => entrada = Some(reg.hora),
                    TipoAsistencia::Salida => salida = Some(reg.hora),
                    TipoAsistencia::EntradaAlmuerzo => inicio_almuerzo = Some(reg.hora),
                    TipoAsistencia::SalidaAlmuerzo => fin_almuerzo = Some(reg.hora),
                }
            }

            // Calcular horas trabajadas del día
            if let (Some(ent), Some(sal)) = (entrada, salida) {
                let segundos_dia = sal.signed_duration_since(ent).num_seconds();
                let horas_dia = segundos_dia as f64 / 3600.0;

                // Restar tiempo de almuerzo si existe
                let horas_almuerzo_dia = if let (Some(ini), Some(fin)) = (inicio_almuerzo, fin_almuerzo) {
                    let seg_almuerzo = fin.signed_duration_since(ini).num_seconds();
                    let h_almuerzo = seg_almuerzo as f64 / 3600.0;
                    horas_almuerzo += h_almuerzo;
                    h_almuerzo
                } else {
                    0.0
                };

                let horas_netas = horas_dia - horas_almuerzo_dia;
                horas_trabajadas += horas_netas.max(0.0);

                // Calcular horas extra (más de 8 horas por día)
                if horas_netas > 8.0 {
                    horas_extra += horas_netas - 8.0;
                }
            }
        }

        Self {
            horas_trabajadas,
            horas_extra,
            horas_almuerzo,
        }
    }
}
