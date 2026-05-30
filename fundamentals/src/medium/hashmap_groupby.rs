/*
  Problem 32: HashMap — Group By First Letter

  Write a function that takes a Vec<String> and returns a HashMap<char, Vec<String>>
  where words are grouped by their first character (lowercase).
  Ignore empty strings.

  Run the tests for this problem with:
    cargo test --test hashmap_groupby_test
*/

use std::collections::HashMap;

pub fn group_by_first_letter(words: Vec<String>) -> HashMap<char, Vec<String>> {
    let mut map = HashMap::new();

    for word in words.iter() {
      if word.is_empty() {
        continue;
      }

      let first = word.chars().next().unwrap().to_ascii_lowercase();

      map.entry(first).or_insert(Vec::new()).push(word.clone());
    }

    map
}
