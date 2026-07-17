use rusqlite::Connection;
use std::path::Path;

pub fn conectar(ruta_db: &Path) -> anyhow::Result<Connection> {
    if let Some(dir) = ruta_db.parent() {
        std::fs::create_dir_all(dir)?;
    }
    let conn = Connection::open(ruta_db)?;
    
    // Configuraciones optimizadas de SQLite
    conn.pragma_update(None, "foreign_keys", true)?;
    conn.pragma_update(None, "journal_mode", "WAL")?;
    conn.pragma_update(None, "synchronous", "NORMAL")?;
    conn.pragma_update(None, "cache_size", -64000)?; // 64MB cache
    conn.pragma_update(None, "temp_store", "MEMORY")?;
    
    aplicar_esquema(&conn)?;
    Ok(conn)
}

fn aplicar_esquema(conn: &Connection) -> anyhow::Result<()> {
    // Usar transacción para asegurar que el esquema se aplique completamente
    let tx = conn.unchecked_transaction()?;
    tx.execute_batch(include_str!("migrations/0001_esquema_inicial.sql"))?;
    tx.commit()?;
    Ok(())
}

/// Ejecuta una operación dentro de una transacción con rollback automático en caso de error
pub fn ejecutar_con_transaccion<F, R>(conn: &Connection, operacion: F) -> anyhow::Result<R>
where
    F: FnOnce(&Connection) -> anyhow::Result<R>,
{
    let tx = conn.unchecked_transaction()?;
    let resultado = operacion(&tx)?;
    tx.commit()?;
    Ok(resultado)
}
