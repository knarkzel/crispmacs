use crisp::Context;
use eframe::{egui::*, epi};

#[derive(Default)]
pub struct Editor {
    input: String,
    context: Context,
    output: String,
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
        for font in fonts.family_and_size.iter_mut() {
            *font.1 = (FontFamily::Proportional, 48.0);
        }
        ctx.set_fonts(fonts);

        // Light mode
        ctx.set_visuals(Visuals::light());
    }

    fn update(&mut self, ctx: &CtxRef, _: &epi::Frame) {
        TopBottomPanel::top("Menu").show(ctx, |ui| {
            if ui.button("Evaluate").clicked() {
                match crisp::parse_and_eval(&self.input, &mut self.context) {
                    Ok(Ok(exprs)) => {
                        self.output = exprs
                            .into_iter()
                            .enumerate()
                            .filter(|(_, it)| *it != crisp::Expr::Nil)
                            .map(|(i, it)| {
                                if i == 0 {
                                    format!("{it}")
                                } else {
                                    format!("{it}\n")
                                }
                            })
                            .collect::<String>()
                    }
                    Ok(Err(error)) => self.output = format!("Evaluation error: {error}"),
                    Err(error) => self.output = format!("Parsing error: {error}"),
                }
            }
        });
        CentralPanel::default().show(ctx, |ui| {
            if self.output.len() > 0 {
                Window::new("Output")
                    .auto_sized()
                    .show(ctx, |ui| {
                        ScrollArea::vertical().show(ui, |ui| {
                            ui.label(&self.output);
                        })
                    });
            }

            ui.add(
                TextEdit::multiline(&mut self.input)
                    .code_editor()
                    .frame(false)
                    .desired_rows(30)
                    .desired_width(f32::INFINITY),
            );
        });
    }
}
