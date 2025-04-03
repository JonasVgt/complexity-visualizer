use egui::{text::LayoutJob, Align, FontSelection, Label, RichText, Style, TextStyle};

#[macro_export]
macro_rules! rich_label {
    ($input:expr) => {
        RichTextParser::new().parse($input).into_label()
    };
}

#[macro_export]
macro_rules! rich_label_heading {
    ($input:expr) => {
        RichTextParser::new()
            .parse($input)
            .text_style(egui::TextStyle::Heading)
            .into_label()
    };
}

#[derive(PartialEq, Eq, Clone, Debug)]
enum RichTextToken {
    Normal,
    Superscript,
}

pub struct RichTextParser {
    tokens: Vec<(RichTextToken, String)>,
    text_style: Option<TextStyle>,
}

impl RichTextParser {
    pub fn new() -> Self {
        Self {
            tokens: Vec::new(),
            text_style: None,
        }
    }

    pub fn text_style(mut self, text_style: TextStyle) -> Self {
        self.text_style = Some(text_style);
        self
    }

    pub fn parse(mut self, input: String) -> Self {
        let mut chars = input.chars();

        while let Some(c) = chars.next() {
            match c {
                '\\' => {
                    // Escape next character
                    self.push_char(RichTextToken::Normal, Some(c))
                }
                '^' => {
                    // Super script
                    self.push_char(RichTextToken::Superscript, chars.next());
                }
                c => self.push_char(RichTextToken::Normal, Some(c)),
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

    pub fn into_layout(self) -> LayoutJob {
        let mut layout_job = LayoutJob::default();
        let style = Style::default();
        for token in self.tokens {
            match token {
                (RichTextToken::Superscript, s) => RichText::new(s).small_raised(),
                (RichTextToken::Normal, s) if self.text_style.is_some() => {
                    RichText::new(s).text_style(self.text_style.clone().unwrap())
                }
                (RichTextToken::Normal, s) => RichText::new(s),
            }
            .append_to(
                &mut layout_job,
                &style,
                FontSelection::Default,
                Align::Center,
            );
        }
        layout_job
    }

    pub fn into_label(self) -> Label {
        Label::new(self.into_layout())
    }
}
