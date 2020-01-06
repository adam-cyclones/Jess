
use super::super::common::regex::valid_name;

// Rules
// is word
// cannot starts with number
// can contain numbers
// can start with underscore
// can contain only underscores
// is not quoted
// MUST match all to end of input $

pub fn matches_common_name (source: &str) -> (&str, bool) {
  let re = valid_name();
  let is_match = re.is_match(source);
  return (
      source,
      is_match
  );
}

#[cfg(test)]
mod tests {
  use super::*;
  // Names
  #[test]
  fn test_common_name_name_starts_with_number() {
      let starts_with_number: &str = "4ello";
      let expected: bool = false;
      let (
          _input,
          actual
      ) = matches_common_name(starts_with_number);
      assert_eq!(expected, actual);
  }

  #[test]
  fn test_common_name_name_contains_number_not_starts_with() {
      let contains_number: &str = "he1lo";
      let expected: bool = true;
      let (
          _input,
          actual
      ) = matches_common_name(contains_number);
      assert_eq!(expected, actual);
  }

  #[test]
  fn test_common_name_name_contains_underscores() {
      let contains_underscores: &str = "h_e1lo";
      let expected: bool = true;
      let (
          _input,
          actual
      ) = matches_common_name(contains_underscores);
      assert_eq!(expected, actual);
  }

  #[test]
  fn test_common_name_name_starts_with_underscores() {
      let starts_with_underscore: &str = "_he1lo";
      let expected: bool = true;
      let (
          _input,
          actual
      ) = matches_common_name(starts_with_underscore);
      assert_eq!(expected, actual);
  }

  #[test]
  fn test_common_name_name_starts_with_caps() {
      let starts_with_cap: &str = "Che1lo";
      let expected: bool = true;
      let (
          _input,
          actual
      ) = matches_common_name(starts_with_cap);
      assert_eq!(expected, actual);
  }

  #[test]
  fn test_common_name_name_contains_caps() {
      let contains_cap: &str = "ChE1LoL";
      let expected: bool = true;
      let (
          _input,
          actual
      ) = matches_common_name(contains_cap);
      assert_eq!(expected, actual);
  }

  #[test]
  fn test_common_name_mixed_valid() {
      let mixed: &str = "__ChE31L_oL2_";
      let expected: bool = true;
      let (
          _input,
          actual
      ) = matches_common_name(mixed);
      assert_eq!(expected, actual);
  }

  #[test]
  fn test_common_name_mixed_invalid() {
      let mixed: &str = "_Ch-E.3&$1+L_oL2_";
      let expected: bool = false;
      let (
          _input,
          actual
      ) = matches_common_name(mixed);
      assert_eq!(expected, actual);
  }
}