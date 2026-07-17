use lopdf::Document;
use std::path::Path;

/// Extrae texto plano de todas las páginas de un PDF. Útil para indexar
/// o previsualizar documentos cargados (actas, certificados, etc.).
pub fn extraer_texto(ruta: &Path) -> anyhow::Result<String> {
    let doc = Document::load(ruta)?;
    let mut texto_completo = String::new();

    for (numero_pagina, _) in doc.get_pages() {
        if let Ok(texto) = doc.extract_text(&[numero_pagina]) {
            texto_completo.push_str(&texto);
            texto_completo.push('\n');
        }
    }

    Ok(texto_completo)
}

/// Número de páginas de un PDF (útil para mostrar en la UI antes de abrirlo).
pub fn contar_paginas(ruta: &Path) -> anyhow::Result<usize> {
    let doc = Document::load(ruta)?;
    Ok(doc.get_pages().len())
}
