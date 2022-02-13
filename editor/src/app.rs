use eframe::{egui::*, epi};

#[derive(Default)]
pub struct Editor {
    input: String,
}

impl epi::App for Editor {
    fn name(&self) -> &str {
        "crispmacs"
    }

    fn max_size_points(&self) -> Vec2 {
        Vec2::new(2560.0, 1440.0)
    }

    fn update(&mut self, ctx: &CtxRef, _: &epi::Frame) {
        let mut fonts = FontDefinitions::default();

        fonts
            .family_and_size
            .insert(TextStyle::Monospace, (FontFamily::Proportional, 48.0));

        ctx.set_fonts(fonts);

        CentralPanel::default().show(ctx, |ui| {
            ui.add_sized(
                ui.available_size(),
                TextEdit::multiline(&mut self.input)
                    .code_editor()
                    .frame(false),
            );
        });
    }
}
