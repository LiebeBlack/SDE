use app_core::models::Estudiante;
use printpdf::*;
use std::fs::File;
use std::	io::BufWriter;
use std::path::Path;

/// Genera una constancia de estudios en PDF para un estudiante y la guarda
/// en `destino`. Devuelve la ruta final del archivo.
pub fn generar_constancia_estudios(
    estudiante: &Estudiante,
    nombre_institucion: &str,
    destino: &Path,
) -> anyhow::Result<()> {
    let (doc, page1, layer1) =
        PdfDocument::new("Constancia de Estudios", Mm(215.9), Mm(279.4), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);

    let font = doc.add_builtin_font(BuiltinFont::HelveticaBold)?;
    let font_normal = doc.add_builtin_font(BuiltinFont::Helvetica)?;

    current_layer.use_text(nombre_institucion, 18.0, Mm(20.0), Mm(260.0), &font);
    current_layer.use_text(
        "CONSTANCIA DE ESTUDIOS",
        14.0,
        Mm(20.0),
        Mm(245.0),
        &font,
    );

    let cuerpo = format!(
        "Se hace constar que {} (matrícula {}) se encuentra actualmente \
         inscrito(a) en {}, con estado: {}.",
        estudiante.nombre_completo(),
        estudiante.matricula,
        estudiante.grado_nivel,
        estudiante.estado
    );

    current_layer.use_text(cuerpo, 11.0, Mm(20.0), Mm(220.0), &font_normal);

    let fecha = chrono::Local::now().format("%d/%m/%Y").to_string();
    current_layer.use_text(
        format!("Emitido el {fecha}"),
        10.0,
        Mm(20.0),
        Mm(40.0),
        &font_normal,
    );

    if let Some(dir) = destino.parent() {
        std::fs::create_dir_all(dir)?;
    }
    doc.save(&mut BufWriter::new(File::create(destino)?))?;
    Ok(())
}
