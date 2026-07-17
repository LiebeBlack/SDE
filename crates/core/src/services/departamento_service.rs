use crate::models::Departamento;
use crate::validation::validar_no_vacio;
use uuid::Uuid;

pub trait DepartamentoRepositorio {
    fn guardar(&self, departamento: &Departamento) -> anyhow::Result<()>;
    fn buscar_por_id(&self, id: Uuid) -> anyhow::Result<Option<Departamento>>;
    fn listar_todos(&self) -> anyhow::Result<Vec<Departamento>>;
    fn eliminar(&self, id: Uuid) -> anyhow::Result<()>;
}

pub struct DepartamentoService<'a, R: DepartamentoRepositorio> {
    repo: &'a R,
}

impl<'a, R: DepartamentoRepositorio> DepartamentoService<'a, R> {
    pub fn new(repo: &'a R) -> Self {
        Self { repo }
    }

    pub fn crear(&self, departamento: Departamento) -> anyhow::Result<Departamento> {
        validar_no_vacio("nombre", &departamento.nombre)?;
        self.repo.guardar(&departamento)?;
        Ok(departamento)
    }

    pub fn listar(&self) -> anyhow::Result<Vec<Departamento>> {
        self.repo.listar_todos()
    }

    pub fn buscar_por_id(&self, id: Uuid) -> anyhow::Result<Option<Departamento>> {
        self.repo.buscar_por_id(id)
    }

    pub fn eliminar(&self, id: Uuid) -> anyhow::Result<()> {
        self.repo.eliminar(id)
    }
}
