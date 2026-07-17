pub mod estudiante;
pub mod familiar;
pub mod documento;
pub mod departamento;
pub mod empleado;
pub mod auditoria;

pub use estudiante::Estudiante;
pub use familiar::{Familiar, Parentesco, RelacionFamiliar};
pub use documento::{Documento, EntidadTipo, TipoDocumento};
pub use departamento::Departamento;
pub use empleado::{Empleado, EstadoEmpleado, TipoContrato, RegistroAsistencia, TipoAsistencia, CalculoHoras};
pub use auditoria::RegistroAuditoria;
