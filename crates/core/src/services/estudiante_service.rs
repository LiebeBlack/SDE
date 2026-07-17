use crate::models::Estudiante;
use crate::validation::validar_no_vacio;
use uuid::Uuid;

/// El core no sabe nada de SQLite: solo define QUÉ necesita.
/// La crate `storage` implementa este trait concretamente.
pub trait EstudianteRepositorio {
    fn guardar(&self, estudiante: &Estudiante) -> anyhow::Result<()>;
    fn buscar_por_id(&self, id: Uuid) -> anyhow::Result<Option<Estudiante>>;
    fn listar_todos(&self) -> anyhow::Result<Vec<Estudiante>>;
    fn buscar_por_nombre(&self, texto: &str) -> anyhow::Result<Vec<Estudiante>>;
    fn eliminar(&self, id: Uuid) -> anyhow::Result<()>;
}

pub struct EstudianteService<'a, R: EstudianteRepositorio> {
    repo: &'a R,
}

impl<'a, R: EstudianteRepositorio> EstudianteService<'a, R> {
    pub fn new(repo: &'a R) -> Self {
        Self { repo }
    }

    pub fn crear(&self, estudiante: Estudiante) -> anyhow::Result<Estudiante> {
        validar_no_vacio("nombre", &estudiante.nombre)?;
        validar_no_vacio("apellido", &estudiante.apellido)?;
        validar_no_vacio("matricula", &estudiante.matricula)?;
        self.repo.guardar(&estudiante)?;
        Ok(estudiante)
    }

    pub fn listar(&self) -> anyhow::Result<Vec<Estudiante>> {
        self.repo.listar_todos()
    }

    pub fn buscar(&self, texto: &str) -> anyhow::Result<Vec<Estudiante>> {
        if texto.trim().is_empty() {
            self.repo.listar_todos()
        } else {
            self.repo.buscar_por_nombre(texto)
        }
    }

    pub fn eliminar(&self, id: Uuid) -> anyhow::Result<()> {
        self.repo.eliminar(id)
    }
}
