pub mod estudiante_service;
pub mod documento_service;
pub mod familiar_service;
pub mod departamento_service;
pub mod empleado_service;

pub use estudiante_service::EstudianteService;
pub use documento_service::DocumentoService;
pub use familiar_service::{FamiliarService, FamiliarRepositorio, RelacionFamiliarRepositorio};
pub use departamento_service::{DepartamentoService, DepartamentoRepositorio};
pub use empleado_service::{EmpleadoService, EmpleadoRepositorio, AsistenciaRepositorio};
