use regex::Regex;

// Patterns

fn content_between_pairs_pattern (open: &str, close: &str) -> String {
  return format!(r"(?:{open_pair}\s*)*{contentMatcher}(?:\s*{close_pair})$",
    open_pair = format!(r"\{token}", token = open),
    close_pair = format!(r"\{token}", token = close),
    contentMatcher = format!(r"((?:\w*,?\s*?)*)")
  );
}

fn content_between_single_line_quotes_pattern () -> &'static str {
  return "^(?:\")(?:[^\"\\\\]|\\\\.)*(?:\")$|^(?:')(?:[^'\\\\]|\\\\.)*(?:')$";
}

fn css_directive_pattern () -> &'static str {
  return r"^(?:\s*@|@)?(\w+)";
}

// an optional white space, {a} { a}
fn whitespace_optional_pattern () -> &'static str {
  return r"(\s*|)+";
}

// used in variables functions and others
fn valid_name_pattern () -> &'static str {
  return r"^[a-zA-Z_$][a-zA-Z_$0-9]*$";
}

// Regex
pub fn valid_name () -> Regex {
  return Regex::new(valid_name_pattern()).unwrap()
}

pub fn content_between_pairs (open: &str, close: &str) -> Regex {
  return Regex::new(&content_between_pairs_pattern(open, close)).unwrap();
}

pub fn between_curly_braces () -> Regex {
  return content_between_pairs("{", "}");
}

pub fn single_line_quoted () -> Regex {
  return Regex::new(content_between_single_line_quotes_pattern()).unwrap();
}

pub fn css_directive () -> Regex {
  return Regex::new(css_directive_pattern()).unwrap();
}