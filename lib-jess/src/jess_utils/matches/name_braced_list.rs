use super::super::matches::common_name::matches_common_name;
use super::super::common::regex::{
  between_curly_braces
};

// Rules
// is word
// cannot starts with number
// can contain numbers
// can start with underscore
// can contain only underscores
// is not quoted
// must be wrapped in { ..., ..., }
// MUST match all to end of input $


pub fn matches_common_curly_braced_list_comma_sep (source: &str) -> (&str, bool) {
  let re = between_curly_braces();
  let is_match = re.is_match(source);
  return (
      source,
      is_match
  );
}

pub fn capture_common_braced_list (source: &str) -> (&str, Vec<String>) {
  let mut results: Vec<String> = vec!();
  let re = between_curly_braces();
  let cap = re.captures(source).unwrap();
  let brace_content = &cap[1];
  for word in brace_content.split(",") {
    let name = String::from(String::from(word.trim()));
    let (_input, is_valid) = matches_common_name(&name);
    if is_valid {
      results.push( name );
    } else {
      // TODO: ERROR
    }
  }
  return (source, results);
}

#[cfg(test)]
mod tests {
  use super::*;
  // simple boolean
  #[test]
  fn test_common_matches_braced_list() {
    let basic_list: &str = "{ a, b, cbc }";
    let expected: bool = true;
    let (
        _input,
        actual
    ) = matches_common_curly_braced_list_comma_sep(basic_list);
    assert_eq!(expected, actual);
  }

  #[test]
  fn test_common_matches_braced_list_multilne() {
    let multline: &str = r#"{
      a,
      b,
      c
    }"#;
    let expected: bool = true;
    let (
        _input,
        actual
    ) = matches_common_curly_braced_list_comma_sep(multline);
    assert_eq!(expected, actual);
  }

  #[test]
  fn test_common_matches_braced_list_crushed_whitespace() {
    let basic_list: &str = r#"{apple,b,c}"#;
    let expected: bool = true;
    let (
        _input,
        actual
    ) = matches_common_curly_braced_list_comma_sep(basic_list);
    assert_eq!(expected, actual);
  }

  // Capture groups aka content within braces
  // Names should validate
  #[test]
  fn test_common_capture_braced_list() {
    let basic_list: &str = r#"{red, yellow, green}"#;
    let expected = vec!("red", "yellow", "green");
    let (
        _input,
        actual
    ) = capture_common_braced_list(basic_list);
    assert_eq!(expected, actual);
  }

  #[test]
  fn test_common_capture_braced_list_multiline() {
    let basic_list: &str = r#"{
      red,
      yellow,
      green
    }"#;
    let expected = vec!("red", "yellow", "green");
    let (
        _input,
        actual
    ) = capture_common_braced_list(basic_list);
    assert_eq!(expected, actual);
  }

  #[test]
  fn test_common_capture_braced_list_valid_invalid_names() {
    let basic_list: &str = r#"{
      red_1_2,
      1yellow,
      _green
    }"#;
    let expected = vec!("red_1_2", "_green");
    let (
        _input,
        actual
    ) = capture_common_braced_list(basic_list);
    assert_eq!(expected, actual);
  }
}