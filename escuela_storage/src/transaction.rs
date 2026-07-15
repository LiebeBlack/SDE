//! Manejo de transacciones para operaciones complejas
//! Proporciona atomicidad para operaciones que deben ejecutarse completamente o fallar completamente

use sqlx::{SqlitePool, Transaction};
use escuela_shared::{AppResult, AppError};

/// Wrapper para transacciones de base de datos
pub struct DatabaseTransaction<'a> {
    transaction: Transaction<'a, sqlx::Sqlite>,
}

impl<'a> DatabaseTransaction<'a> {
    /// Crea una nueva transacción desde el pool
    pub async fn begin(pool: &'a SqlitePool) -> AppResult<Self> {
        let transaction = pool.begin().await
            .map_err(|e| AppError::DatabaseError(format!("Error al iniciar transacción: {}", e)))?;
        
        Ok(DatabaseTransaction { transaction })
    }

    /// Obtiene la transacción subyacente para usar con repositorios
    pub fn transaction(&mut self) -> &mut Transaction<'a, sqlx::Sqlite> {
        &mut self.transaction
    }

    /// Confirma la transacción (commit)
    pub async fn commit(self) -> AppResult<()> {
        self.transaction.commit().await
            .map_err(|e| AppError::DatabaseError(format!("Error al confirmar transacción: {}", e)))
    }

    /// Revierte la transacción (rollback)
    pub async fn rollback(self) -> AppResult<()> {
        self.transaction.rollback().await
            .map_err(|e| AppError::DatabaseError(format!("Error al revertir transacción: {}", e)))
    }
}

/// Ejecuta una operación dentro de una transacción con manejo automático de rollback en caso de error
pub async fn execute_in_transaction<F, T, Fut>(
    pool: &SqlitePool,
    operation: F,
) -> AppResult<T>
where
    F: FnOnce(&mut Transaction<'_, sqlx::Sqlite>) -> Fut,
    Fut: std::future::Future<Output = AppResult<T>>,
{
    let mut transaction = pool.begin().await
        .map_err(|e| AppError::DatabaseError(format!("Error al iniciar transacción: {}", e)))?;
    
    match operation(&mut transaction).await {
        Ok(result) => {
            transaction.commit().await
                .map_err(|e| AppError::DatabaseError(format!("Error al confirmar transacción: {}", e)))?;
            Ok(result)
        }
        Err(e) => {
            transaction.rollback().await
                .map_err(|err| AppError::DatabaseError(format!("Error al revertir transacción: {}", err)))?;
            Err(e)
        }
    }
}
