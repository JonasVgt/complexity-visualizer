use egui::{text::LayoutJob, Align, FontSelection, RichText, Style};

#[derive(PartialEq, Eq, Clone, Debug)]
enum RichTextToken {
    NORMAL,
    SUPERSCRIPT,
}

pub struct RichTextParser {
    tokens: Vec<(RichTextToken, String)>,
}

impl RichTextParser {
    pub fn new() -> Self {
        Self { tokens: Vec::new() }
    }

    pub fn parse(mut self, input: String) -> Self {
        let mut chars = input.chars();

        while let Some(c) = chars.next() {
            match c {
                '\\' => {
                    // Escape next character
                    self.push_char(RichTextToken::NORMAL, Some(c))
                }
                '^' => {
                    // Super script
                    self.push_char(RichTextToken::SUPERSCRIPT, chars.next());
                }
                c => self.push_char(RichTextToken::NORMAL, Some(c)),
            }
        }
        self
    }

    fn push_char(&mut self, token: RichTextToken, c: Option<char>) {
        if c.is_none() {
            return;
        }

        if let Some((t, mut s)) = self.tokens.pop() {
            if t == token {
                s.push(c.unwrap());
                self.tokens.push((t, s));
                return;
            }
            self.tokens.push((t, s));
        }
        self.tokens.push((token, String::from(c.unwrap())));
    }

    pub fn to_layout(self) -> LayoutJob {
        let mut layout_job = LayoutJob::default();
        let style = Style::default();
        for token in self.tokens {
            match token {
                (RichTextToken::SUPERSCRIPT, s) => RichText::new(s).small_raised(),
                (RichTextToken::NORMAL, s) => RichText::new(s),
            }
            .append_to(
                &mut layout_job,
                &style,
                FontSelection::Default,
                Align::Center,
            );
        }
        return layout_job;
    }
}
