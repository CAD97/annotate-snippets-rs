use display_list::{DisplayAnnotationType, DisplayLine, DisplayList, DisplayMark,
                   DisplaySnippetType};
use display_list_formatting::DisplayListFormatting;
use std::fmt;

struct Formatter {}

impl DisplayListFormatting for Formatter {
    fn format_snippet_type(snippet_type: &DisplaySnippetType) -> String {
        match snippet_type {
            DisplaySnippetType::Error => "error".to_string(),
            DisplaySnippetType::Warning => "warning".to_string(),
        }
    }

    fn format_inline_marks(inline_marks: &[DisplayMark], inline_marks_width: usize) -> String {
        format!(
            "{:>width$}",
            inline_marks
                .iter()
                .map(|mark| match mark {
                    DisplayMark::AnnotationThrough => "|",
                    DisplayMark::AnnotationStart => "/",
                })
                .collect::<Vec<&str>>()
                .join(""),
            width = inline_marks_width
        )
    }

    fn format_annotation_content(
        range: &(usize, usize),
        label: &Option<String>,
        annotation_type: &DisplayAnnotationType,
    ) -> String {
        let label = label.clone().map_or("".to_string(), |l| format!(" {}", l));
        match annotation_type {
            DisplayAnnotationType::Error => format!(
                "{}{}{}",
                " ".repeat(range.0),
                "^".repeat(range.1 - range.0),
                label
            ),
            DisplayAnnotationType::Warning => format!(
                "{}{}{}",
                " ".repeat(range.0),
                "-".repeat(range.1 - range.0),
                label
            ),
            DisplayAnnotationType::MultilineStart => format!(
                "{}{}{}",
                "_".repeat(range.0),
                "^".repeat(range.1 - range.0),
                label
            ),
            DisplayAnnotationType::MultilineEnd => format!(
                "{}{}{}",
                "_".repeat(range.0),
                "^".repeat(range.1 - range.0),
                label
            ),
        }
    }

    fn format_line(
        f: &mut fmt::Formatter,
        dl: &DisplayLine,
        lineno_width: usize,
        inline_marks_width: usize,
    ) -> fmt::Result {
        match dl {
            DisplayLine::Description {
                snippet_type,
                id,
                label,
            } => writeln!(
                f,
                "{}[{}]: {}",
                Self::format_snippet_type(&snippet_type),
                id,
                label
            ),
            DisplayLine::Origin { path, row, col } => writeln!(
                f,
                "{}--> {}:{}:{}",
                " ".repeat(lineno_width),
                path,
                row,
                col
            ),
            DisplayLine::EmptySource => writeln!(f, "{} |", " ".repeat(lineno_width)),
            DisplayLine::Source {
                lineno,
                inline_marks,
                content,
                ..
            } => writeln!(
                f,
                "{:>width$} |{} {}",
                lineno,
                Self::format_inline_marks(&inline_marks, inline_marks_width),
                content,
                width = lineno_width,
            ),
            DisplayLine::Annotation {
                inline_marks,
                range,
                label,
                annotation_type,
            } => writeln!(
                f,
                "{} |{}{}",
                " ".repeat(lineno_width),
                Self::format_inline_marks(&inline_marks, inline_marks_width),
                Self::format_annotation_content(range, &label, &annotation_type),
            ),
            DisplayLine::Fold { inline_marks } => writeln!(
                f,
                "... {}",
                Self::format_inline_marks(&inline_marks, inline_marks_width),
            ),
        }
    }
}

impl fmt::Display for DisplayList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let lineno_width = self.body.iter().fold(0, |max, line| match line {
            DisplayLine::Source { lineno, .. } => {
                let width = lineno.to_string().len();
                if width > max {
                    width
                } else {
                    max
                }
            }
            _ => max,
        });
        let inline_marks_width = self.body.iter().fold(0, |max, line| match line {
            DisplayLine::Source { inline_marks, .. } => {
                let width = inline_marks.len();
                if width > max {
                    width + 1
                } else {
                    max
                }
            }
            _ => max,
        });

        for line in &self.body {
            Formatter::format_line(f, line, lineno_width, inline_marks_width)?;
        }
        Ok(())
    }
}
