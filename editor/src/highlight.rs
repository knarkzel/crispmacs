use eframe::egui::{self, text::LayoutJob};
use egui::text::{LayoutSection, TextFormat};
use syntect::{easy::HighlightLines, highlighting::FontStyle, util::LinesWithEndings};

/// Memoized Code highlighting
pub fn highlight(ctx: &egui::Context, code: &str, language: &str) -> LayoutJob {
    impl egui::util::cache::ComputerMut<(&str, &str), LayoutJob> for Highligher {
        fn compute(&mut self, (code, lang): (&str, &str)) -> LayoutJob {
            self.highlight(code, lang)
        }
    }
    type HighlightCache<'a> = egui::util::cache::FrameCache<LayoutJob, Highligher>;
    let mut memory = ctx.memory();
    let highlight_cache = memory.caches.cache::<HighlightCache<'_>>();
    highlight_cache.get((code, language))
}

struct Highligher {
    syntax_set: syntect::parsing::SyntaxSet,
    theme_set: syntect::highlighting::ThemeSet,
}

impl Default for Highligher {
    fn default() -> Self {
        Self {
            syntax_set: syntect::parsing::SyntaxSet::load_defaults_newlines(),
            theme_set: syntect::highlighting::ThemeSet::load_defaults(),
        }
    }
}

impl Highligher {
    fn highlight(&self, input: &str, language: &str) -> LayoutJob {
        let syntax = self
            .syntax_set
            .find_syntax_by_name(language)
            .or_else(|| self.syntax_set.find_syntax_by_extension(language)).expect("Invalid language passed");
        let mut lines = HighlightLines::new(syntax, &self.theme_set.themes["Solarized (light)"]);
        let mut job = LayoutJob {
            text: input.into(),
            ..Default::default()
        };
        for line in LinesWithEndings::from(input) {
            for (style, range) in lines.highlight(line, &self.syntax_set) {
                let fg = style.foreground;
                let text_color = egui::Color32::from_rgb(fg.r, fg.g, fg.b);
                let italics = style.font_style.contains(FontStyle::ITALIC);
                let underline = style.font_style.contains(FontStyle::ITALIC);
                let underline = if underline {
                    egui::Stroke::new(1.0, text_color)
                } else {
                    egui::Stroke::none()
                };
                job.sections.push(LayoutSection {
                    leading_space: 0.0,
                    byte_range: as_byte_range(input, range),
                    format: TextFormat {
                        color: text_color,
                        italics,
                        underline,
                        ..Default::default()
                    },
                });
            }
        }
        job
    }
}

fn as_byte_range(whole: &str, range: &str) -> std::ops::Range<usize> {
    let whole_start = whole.as_ptr() as usize;
    let range_start = range.as_ptr() as usize;
    assert!(whole_start <= range_start);
    assert!(range_start + range.len() <= whole_start + whole.len());
    let offset = range_start - whole_start;
    offset..(offset + range.len())
}
