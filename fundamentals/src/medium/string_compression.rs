/*
  Problem 33: String Compression

  Write a function that performs basic string compression using the counts of repeated characters.
  For example, "aabcccccaaa" becomes "a2b1c5a3".
  If the compressed string is not shorter than the original, return the original string.

  Run the tests for this problem with:
    cargo test --test string_compression_test
*/

pub fn compress(s: &str) -> String {

    if s.is_empty() {
      return String::new();
    }

    let mut compressed = String::new();

    let mut chars = s.chars();
    let mut prev = chars.next().unwrap();

    let mut cnt = 1;

    for c in chars {
      if c==prev {
        cnt+=1;
      } else {
        compressed.push(prev);
        compressed.push_str(&cnt.to_string());

        prev = c;
        cnt = 1;
      }
    }

    compressed.push(prev);
    compressed.push_str(&cnt.to_string());

    if compressed.len() <= s.len() {
      compressed
    } else {
      s.to_string()
    }
}
