use egui::{Context, Style, Visuals};

pub fn aplicar_tema(ctx: &Context) {
    // Tema institucional profesional con colores institucionales
    let mut visuals = Visuals::light();
    
    // Colores base
    visuals.panel_fill = egui::Color32::from_rgb(245, 247, 250);
    visuals.window_fill = egui::Color32::from_rgb(255, 255, 255);
    visuals.extreme_bg_color = egui::Color32::from_rgb(230, 233, 239);
    
    // Colores de widgets
    visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(248, 250, 252);
    visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(59, 130, 246); // Azul institucional
    visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(37, 99, 235); // Azul más oscuro
    visuals.widgets.active.bg_fill = egui::Color32::from_rgb(29, 78, 216); // Azul más oscuro aún
    
    // Colores de texto
    visuals.widgets.noninteractive.fg_stroke.color = egui::Color32::from_rgb(51, 65, 85);
    visuals.widgets.inactive.fg_stroke.color = egui::Color32::from_rgb(255, 255, 255);
    visuals.widgets.hovered.fg_stroke.color = egui::Color32::from_rgb(255, 255, 255);
    visuals.widgets.active.fg_stroke.color = egui::Color32::from_rgb(255, 255, 255);
    
    // Colores de selección
    visuals.selection.bg_fill = egui::Color32::from_rgb(59, 130, 246);
    visuals.selection.stroke.color = egui::Color32::from_rgb(37, 99, 235);
    
    ctx.set_visuals(visuals);

    // Estilo mejorado con espaciado profesional
    let mut estilo = Style::default();
    
    // Espaciado
    estilo.spacing.item_spacing = egui::vec2(12.0, 10.0);
    estilo.spacing.window_margin = egui::Margin::symmetric(16.0, 16.0);
    estilo.spacing.button_padding = egui::vec2(16.0, 8.0);
    estilo.spacing.indent = 24.0;
    estilo.spacing.interact_size = egui::vec2(40.0, 24.0);
    estilo.spacing.slider_width = 200.0;
    estilo.spacing.text_edit_width = 280.0;
    
    // Texto
    estilo.text_styles = [
        (egui::TextStyle::Heading, egui::FontId::proportional(28.0)),
        (egui::TextStyle::Body, egui::FontId::proportional(14.0)),
        (egui::TextStyle::Button, egui::FontId::proportional(14.0)),
        (egui::TextStyle::Small, egui::FontId::proportional(11.0)),
        (egui::TextStyle::Monospace, egui::FontId::monospace(13.0)),
    ].into();
    
    // Bordes y esquinas
    estilo.visuals.widgets.noninteractive.rounding = egui::Rounding::same(6.0);
    estilo.visuals.widgets.inactive.rounding = egui::Rounding::same(6.0);
    estilo.visuals.widgets.hovered.rounding = egui::Rounding::same(6.0);
    estilo.visuals.widgets.active.rounding = egui::Rounding::same(6.0);
    
    estilo.visuals.widgets.inactive.expansion = 0.0;
    estilo.visuals.widgets.hovered.expansion = 1.0;
    estilo.visuals.widgets.active.expansion = 2.0;
    
    ctx.set_style(estilo);
}

pub fn color_estado_activo() -> egui::Color32 {
    egui::Color32::from_rgb(34, 197, 94) // Verde
}

pub fn color_estado_inactivo() -> egui::Color32 {
    egui::Color32::from_rgb(239, 68, 68) // Rojo
}

pub fn color_estado_pendiente() -> egui::Color32 {
    egui::Color32::from_rgb(234, 179, 8) // Amarillo
}

pub fn color_primario() -> egui::Color32 {
    egui::Color32::from_rgb(59, 130, 246) // Azul institucional
}

pub fn color_fondo() -> egui::Color32 {
    egui::Color32::from_rgb(245, 247, 250)
}

