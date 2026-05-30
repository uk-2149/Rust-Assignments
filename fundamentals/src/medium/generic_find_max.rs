/*
  Problem 28: Generic — Find Max

  Write a generic function that takes a slice of items implementing PartialOrd
  and returns Option<&T> for the maximum element.
  Do not use any built-in max functions.

  Run the tests for this problem with:
    cargo test --test generic_find_max_test
*/

pub fn find_max<T: PartialOrd>(items: &[T]) -> Option<&T> {
    if items.is_empty() {
        return None;
    }

    // let mut iter = items.iter();

    // let mut mx = iter.next().unwrap();

    // loop {
    //     if let Some(temp) = iter.next() {
    //         if temp > mx {
    //             mx = temp;
    //         } else {
    //             continue;
    //         }
    //     } else {
    //         break;
    //     }
    // }

    // Some(mx)

    let mut mx = items.iter().next().unwrap();

    for val in items {
      if mx < val {
        mx = val;
      }
    }

    Some(mx)
}
