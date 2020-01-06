extern crate enquote;

use super::super::common::regex::{
  single_line_quoted
};

// Rules
// anything in quotes
// can be single or double
// can escape with backslack \' or \"
// whole imput must match " hello " " does not match
// whole imput must match " hello "  " world" does not match


pub fn matches_common_quoted (source: &str) -> (&str, bool) {
  let re = single_line_quoted();
  let is_match = re.is_match(source);
  return (
      source,
      is_match
  );
}

pub fn capture_common_quoted (source: &str) -> (&str, String) {
  let re = single_line_quoted();
  let cap = re.captures(source).unwrap();
  let content = String::from(&cap[0]);
  return (source, content);
}

// TODO retain inner quotes
pub fn unquote (source: &str) -> (&str, String) {
  let (_input, content) = capture_common_quoted(source);
  let unquoted_content = enquote::unquote(&content).unwrap();
  return (source, unquoted_content);
}

#[cfg(test)]
mod tests {
  use super::*;

  // unquotes quoted content
  #[test]
  fn test_unquote_double() {
    let text: &str = "\"This is sparta\"";
    let expected: &str = "This is sparta";
    let (
      _input,
      actual
    ) = unquote(text);
    assert_eq!(expected, actual);
  }

  #[test]
  fn test_unquote_single() {
    let text: &str = "'This is also sparta'";
    let expected: &str = "This is also sparta";
    let (
      _input,
      actual
    ) = unquote(text);
    assert_eq!(expected, actual);
  }

  // matches whole quote string for both single and double, including escapes
  #[test]
  fn test_common_matches_quoted_double_standard() {
    let text: &str = "\"hello world\"";
    let expected: bool = true;
    let (
        _input,
        actual
    ) = matches_common_quoted(text);
    assert_eq!(expected, actual);
  }

  #[test]
  fn test_common_not_matches_quoted_double_standard() {
    let text: &str = "\"hello \" world\"";
    let expected: bool = false;
    let (
        _input,
        actual
    ) = matches_common_quoted(text);
    assert_eq!(expected, actual);
  }

  // captures whole quote string for both single and double, including escapes
  #[test]
  fn test_common_capture_quoted_double_standard() {
    let text: &str = "\"hello world\"";
    let expected: &str = "\"hello world\"";
    let (
        _input,
        actual
    ) = capture_common_quoted(text);
    assert_eq!(expected, actual);
  }

  #[test]
  fn test_common_capture_quoted_single_standard() {
    let text: &str = "'hello world'";
    let expected: &str = "'hello world'";
    let (
        _input,
        actual
    ) = capture_common_quoted(text);
    assert_eq!(expected, actual);
  }

  #[test]
  fn test_common_capture_quoted_single_escape() {
    let text: &str = "'hello it\\'s world'";
    let expected: &str = "'hello it\\'s world'";
    let (
        _input,
        actual
    ) = capture_common_quoted(text);
    assert_eq!(expected, actual);
  }

  #[test]
  fn test_common_capture_quoted_double_escape() {
    let text: &str = "\"hello it\\\"s world\"";
    let expected: &str = "\"hello it\\\"s world\"";
    let (
        _input,
        actual
    ) = capture_common_quoted(text);
    assert_eq!(expected, actual);
  }

  #[test]
  fn test_common_capture_quoted_single_double_in_single() {
    let text: &str = "'hello its \"world\"'";
    let expected: &str = "'hello its \"world\"'";
    let (
        _input,
        actual
    ) = capture_common_quoted(text);
    assert_eq!(expected, actual);
  }

  #[test]
  fn test_common_capture_quoted_double_signle_in_double() {
    let text: &str = "\"hello its 'world'\"";
    let expected: &str = "\"hello its 'world'\"";
    let (
        _input,
        actual
    ) = capture_common_quoted(text);
    assert_eq!(expected, actual);
  }

}