use super::CodeLanguage;

/// Lexical coloring for configuration snippets, including incomplete input.
/// Every token is a slice of the original text.
pub(super) fn highlight(code: &str, language: CodeLanguage) -> Vec<(&'static str, String)> {
    use CodeLanguage::*;
    const COMMENT: &str = "text-muted-foreground";
    const NAME: &str = "text-primary font-medium";
    const STRING: &str = "text-success-dark";
    const VALUE: &str = "text-info-dark";

    let mut tokens = Vec::new();
    let mut rest = code;
    let mut in_tag = false;
    while let Some(first) = rest.chars().next() {
        let (end, class) = if language == PlainText {
            (rest.len(), "")
        } else if language == Xml && rest.starts_with("<!--") {
            (rest.find("-->").map_or(rest.len(), |end| end + 3), COMMENT)
        } else if language == Xml && rest.starts_with("<![CDATA[") {
            (rest.find("]]>").map_or(rest.len(), |end| end + 3), STRING)
        } else if language == Xml && first == '<' {
            in_tag = true;
            (1, NAME)
        } else if language == Xml && in_tag && first == '>' {
            in_tag = false;
            (1, NAME)
        } else if language == Xml && !in_tag {
            (rest.find('<').unwrap_or(rest.len()), "")
        } else if matches!(language, Yaml | Toml) && first == '#' {
            (rest.find('\n').unwrap_or(rest.len()), COMMENT)
        } else if first == '"' || (first == '\'' && language != Json) {
            let end = string_end(rest, first, language);
            let key = rest[end..].trim_start().starts_with(match language {
                Json | Yaml => ':',
                Toml | Xml => '=',
                PlainText => unreachable!(),
            });
            (end, if key { NAME } else { STRING })
        } else if first.is_alphanumeric() || matches!(first, '_' | '-' | '+') {
            let end = rest
                .find(|c: char| !(c.is_alphanumeric() || matches!(c, '_' | '-' | '+' | '.')))
                .unwrap_or(rest.len());
            let word = &rest[..end];
            let following = rest[end..].trim_start();
            let class = match language {
                Xml => NAME,
                Yaml if following.starts_with(':') => NAME,
                Toml if following.starts_with('=') => NAME,
                _ if matches!(word, "true" | "false" | "null" | "inf" | "nan")
                    || word
                        .trim_start_matches(['+', '-'])
                        .starts_with(|c: char| c.is_ascii_digit()) =>
                {
                    VALUE
                }
                _ => "",
            };
            (end, class)
        } else {
            (first.len_utf8(), "")
        };
        let (token, remaining) = rest.split_at(end);
        if let Some((_, value)) = tokens.last_mut().filter(|(prior, _)| *prior == class) {
            String::push_str(value, token);
        } else {
            tokens.push((class, token.to_owned()));
        }
        rest = remaining;
    }
    tokens
}

fn string_end(text: &str, quote: char, language: CodeLanguage) -> usize {
    let triple = language == CodeLanguage::Toml
        && text
            .as_bytes()
            .get(..3)
            .is_some_and(|bytes| bytes.iter().all(|b| *b == quote as u8));
    let opening = if triple { 3 } else { 1 };
    let mut escaped = false;
    let mut chars = text[opening..].char_indices().peekable();
    while let Some((offset, c)) = chars.next() {
        if escaped {
            escaped = false;
        } else if c == '\\' && quote == '"' && language != CodeLanguage::Xml {
            escaped = true;
        } else if c == quote {
            let end = opening + offset + 1;
            if triple {
                if text
                    .as_bytes()
                    .get(end..end + 2)
                    .is_some_and(|bytes| bytes.iter().all(|b| *b == quote as u8))
                {
                    return end + 2;
                }
            } else if language == CodeLanguage::Yaml
                && quote == '\''
                && chars.peek().is_some_and(|(_, next)| *next == quote)
            {
                chars.next();
            } else {
                return end;
            }
        }
    }
    text.len()
}
