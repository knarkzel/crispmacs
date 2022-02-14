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

    fn setup(&mut self, ctx: &CtxRef, _: &epi::Frame, _: Option<&dyn epi::Storage>) {
        // Proper font
        let mut fonts = FontDefinitions::default();
        fonts
            .family_and_size
            .insert(TextStyle::Monospace, (FontFamily::Proportional, 48.0));
        ctx.set_fonts(fonts);

        // Light mode
        ctx.set_visuals(Visuals::light());
    }

    fn update(&mut self, ctx: &CtxRef, _: &epi::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                ui.add(
                    TextEdit::multiline(&mut self.input)
                        .code_editor()
                        .frame(false)
                        .desired_rows(usize::MAX)
                        .desired_width(f32::INFINITY),
                );
            })
        });
    }
}
