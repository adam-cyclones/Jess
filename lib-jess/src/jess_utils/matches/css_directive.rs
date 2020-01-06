
use super::super::common::regex::css_directive;

// Rules
// is word preceded by @ symbol
// can be followed by { or ( depending on the directive

pub fn matches_common_css_directive (source: &str) -> (&str, bool) {
  let re = css_directive();
  let is_match = re.is_match(source);
  return (
      source,
      is_match
  );
}

pub fn capture_common_css_directive_name (source: &str) -> (&str, String) {
  let re = css_directive();
  let cap = re.captures(source).unwrap();
  return (
      source,
      String::from(&cap[1])
  );
}

#[cfg(test)]
mod tests {
  use super::*;
  // Directive name capture
  #[test]
  fn test_common_captures_css_directive_name_only() {
      let starts_with_number: &str = "@foo";
      let expected: &str = "foo";
      let (
          _input,
          actual
      ) = capture_common_css_directive_name(starts_with_number);
      assert_eq!(expected, actual);
  }

  #[test]
  fn test_common_captures_css_directive_name_only_nested() {
      let starts_with_number: &str = "  @foo";
      let expected: &str = "foo";
      let (
          _input,
          actual
      ) = capture_common_css_directive_name(starts_with_number);
      assert_eq!(expected, actual);
  }
  // Did match
  #[test]
  fn test_common_matches_css_directive_name_only() {
      let starts_with_number: &str = "@foo";
      let expected: bool = true;
      let (
          _input,
          actual
      ) = matches_common_css_directive(starts_with_number);
      assert_eq!(expected, actual);
  }
  #[test]
  fn test_common_matches_css_directive_name_nested() {
      let starts_with_number: &str = "    @foo";
      let expected: bool = true;
      let (
          _input,
          actual
      ) = matches_common_css_directive(starts_with_number);
      assert_eq!(expected, actual);
  }
}