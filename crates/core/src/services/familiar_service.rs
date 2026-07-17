use crate::models::{Familiar, Parentesco, RelacionFamiliar};
use crate::validation::validar_no_vacio;
use uuid::Uuid;

pub trait FamiliarRepositorio {
    fn guardar(&self, familiar: &Familiar) -> anyhow::Result<()>;
    fn buscar_por_id(&self, id: Uuid) -> anyhow::Result<Option<Familiar>>;
    fn listar_todos(&self) -> anyhow::Result<Vec<Familiar>>;
    fn eliminar(&self, id: Uuid) -> anyhow::Result<()>;
}

pub trait RelacionFamiliarRepositorio {
    fn guardar_relacion(&self, relacion: &RelacionFamiliar) -> anyhow::Result<()>;
    fn listar_por_estudiante(&self, estudiante_id: Uuid) -> anyhow::Result<Vec<RelacionFamiliar>>;
    fn listar_por_familiar(&self, familiar_id: Uuid) -> anyhow::Result<Vec<RelacionFamiliar>>;
    fn eliminar_relacion(&self, id: Uuid) -> anyhow::Result<()>;
}

pub struct FamiliarService<'a, R: FamiliarRepositorio, RR: RelacionFamiliarRepositorio> {
    repo: &'a R,
    repo_relacion: &'a RR,
}

impl<'a, R: FamiliarRepositorio, RR: RelacionFamiliarRepositorio> FamiliarService<'a, R, RR> {
    pub fn new(repo: &'a R, repo_relacion: &'a RR) -> Self {
        Self { repo, repo_relacion }
    }

    pub fn crear(&self, familiar: Familiar) -> anyhow::Result<Familiar> {
        validar_no_vacio("nombre", &familiar.nombre)?;
        validar_no_vacio("apellido", &familiar.apellido)?;
        self.repo.guardar(&familiar)?;
        Ok(familiar)
    }

    pub fn listar(&self) -> anyhow::Result<Vec<Familiar>> {
        self.repo.listar_todos()
    }

    pub fn buscar_por_id(&self, id: Uuid) -> anyhow::Result<Option<Familiar>> {
        self.repo.buscar_por_id(id)
    }

    pub fn eliminar(&self, id: Uuid) -> anyhow::Result<()> {
        self.repo.eliminar(id)
    }

    pub fn vincular_estudiante(
        &self,
        estudiante_id: Uuid,
        familiar_id: Uuid,
        parentesco: Parentesco,
        es_titular: bool,
    ) -> anyhow::Result<RelacionFamiliar> {
        let relacion = RelacionFamiliar {
            id: Uuid::new_v4(),
            estudiante_id,
            familiar_id,
            parentesco,
            es_titular_responsable: es_titular,
        };
        self.repo_relacion.guardar_relacion(&relacion)?;
        Ok(relacion)
    }

    pub fn familiares_de_estudiante(&self, estudiante_id: Uuid) -> anyhow::Result<Vec<RelacionFamiliar>> {
        self.repo_relacion.listar_por_estudiante(estudiante_id)
    }

    pub fn eliminar_relacion(&self, id: Uuid) -> anyhow::Result<()> {
        self.repo_relacion.eliminar_relacion(id)
    }
}
