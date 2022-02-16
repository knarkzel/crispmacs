use crate::highlight::highlight;
use crisp::Context;
use eframe::{egui::*, epi};

const FONT_SIZE: f32 = 44.0;

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
            let family = font.1.0;
            *font.1 = (family, FONT_SIZE);
        }
        ctx.set_fonts(fonts);

        // Light mode
        ctx.set_visuals(Visuals::light());
    }

    fn update(&mut self, ctx: &CtxRef, _: &epi::Frame) {
        TopBottomPanel::top("Menu").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.expand_to_include_y(FONT_SIZE + 20.0);
                if ui.button("Parse").clicked() {
                    match crisp::parse(&self.input) {
                        Ok(exprs) => self.output = format!("{:#?}", exprs),
                        Err(error) => self.output = format!("Parsing error: {error}"),
                    }
                }

                if ui.button("Evaluate").clicked() {
                    match crisp::parse_and_eval(&self.input, &mut self.context) {
                        Ok(Ok(exprs)) => {
                            self.output = exprs
                                .into_iter()
                                .map(|it| format!("{it}"))
                                .collect::<Vec<_>>()
                                .join("\n")
                        }
                        Ok(Err(error)) => self.output = format!("Evaluation error: {error}"),
                        Err(error) => self.output = format!("Parsing error: {error}"),
                    }
                }
            });
        });
        CentralPanel::default().show(ctx, |ui| {
            if self.output.len() > 0 {
                Window::new("Result").auto_sized().show(ctx, |ui| {
                    ScrollArea::vertical().show(ui, |ui| {
                        ui.label(&self.output);
                    })
                });
            }

            let mut layouter = |ui: &Ui, input: &str, wrap_width: f32| {
                let mut job = highlight(ui.ctx(), input, "lisp");
                job.wrap_width = wrap_width;
                ui.fonts().layout_job(job)
            };

            ui.add(
                TextEdit::multiline(&mut self.input)
                    .code_editor()
                    .frame(false)
                    .desired_rows(30)
                    .desired_width(f32::INFINITY)
                    .layouter(&mut layouter),
            );
        });
    }
}
