use eframe::egui;

pub fn apply_theme(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();
    style.visuals = egui::Visuals {
        dark_mode: true,
        override_text_color: Some(egui::Color32::from_rgb(230, 230, 230)),
        widgets: egui::style::Widgets {
            noninteractive: egui::style::WidgetVisuals {
                bg_fill: egui::Color32::from_rgb(27, 27, 27),
                weak_bg_fill: egui::Color32::from_rgb(27, 27, 27),
                bg_stroke: egui::Stroke::new(1.0, egui::Color32::from_rgb(50, 50, 50)),
                fg_stroke: egui::Stroke::new(1.0, egui::Color32::from_rgb(230, 230, 230)),
                rounding: egui::Rounding::same(4.0),
                expansion: 0.0,
            },
            inactive: egui::style::WidgetVisuals {
                bg_fill: egui::Color32::from_rgb(45, 45, 45),
                weak_bg_fill: egui::Color32::from_rgb(45, 45, 45),
                bg_stroke: egui::Stroke::new(1.0, egui::Color32::from_rgb(70, 70, 70)),
                fg_stroke: egui::Stroke::new(1.0, egui::Color32::from_rgb(230, 230, 230)),
                rounding: egui::Rounding::same(4.0),
                expansion: 0.0,
            },
            hovered: egui::style::WidgetVisuals {
                bg_fill: egui::Color32::from_rgb(60, 60, 60),
                weak_bg_fill: egui::Color32::from_rgb(60, 60, 60),
                bg_stroke: egui::Stroke::new(1.0, egui::Color32::from_rgb(80, 80, 80)),
                fg_stroke: egui::Stroke::new(1.0, egui::Color32::from_rgb(255, 255, 255)),
                rounding: egui::Rounding::same(4.0),
                expansion: 0.0,
            },
            active: egui::style::WidgetVisuals {
                bg_fill: egui::Color32::from_rgb(75, 75, 75),
                weak_bg_fill: egui::Color32::from_rgb(75, 75, 75),
                bg_stroke: egui::Stroke::new(1.0, egui::Color32::from_rgb(90, 90, 90)),
                fg_stroke: egui::Stroke::new(1.0, egui::Color32::from_rgb(255, 255, 255)),
                rounding: egui::Rounding::same(4.0),
                expansion: 0.0,
            },
            open: egui::style::WidgetVisuals {
                bg_fill: egui::Color32::from_rgb(45, 45, 45),
                weak_bg_fill: egui::Color32::from_rgb(45, 45, 45),
                bg_stroke: egui::Stroke::new(1.0, egui::Color32::from_rgb(70, 70, 70)),
                fg_stroke: egui::Stroke::new(1.0, egui::Color32::from_rgb(230, 230, 230)),
                rounding: egui::Rounding::same(4.0),
                expansion: 0.0,
            },
        },
        selection: egui::style::Selection {
            bg_fill: egui::Color32::from_rgb(0, 116, 217),
            stroke: egui::Stroke::new(1.0, egui::Color32::from_rgb(255, 255, 255)),
        },
        window_rounding: egui::Rounding::same(6.0),
        
        window_shadow: egui::epaint::Shadow {
            offset: egui::vec2(0.0, 5.0),
            blur: 15.0,
            spread: 0.0,
            color: egui::Color32::from_rgba_unmultiplied(0, 0, 0, 100),
        },
        ..Default::default()
    };

    
    let mut fonts = egui::FontDefinitions::default();

    fonts.font_data.insert(
        "RobotoMono-Regular".to_owned(),
        egui::FontData::from_static(include_bytes!("../assets/fonts/RobotoMono-Regular.ttf")),
    );

    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .insert(0, "RobotoMono-Regular".to_owned());

    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "RobotoMono-Regular".to_owned());

    ctx.set_fonts(fonts);

    let mut style = (*ctx.style()).clone();
    style.text_styles = [
        (egui::TextStyle::Heading, egui::FontId::new(18.0, egui::FontFamily::Proportional)),
        (egui::TextStyle::Body, egui::FontId::new(14.0, egui::FontFamily::Proportional)),
        (egui::TextStyle::Monospace, egui::FontId::new(14.0, egui::FontFamily::Monospace)),
        (egui::TextStyle::Button, egui::FontId::new(16.0, egui::FontFamily::Proportional)),
        (egui::TextStyle::Small, egui::FontId::new(12.0, egui::FontFamily::Proportional)),
    ]
    .into();

    ctx.set_style(style);
}
