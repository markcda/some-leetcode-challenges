#[cfg(test)]
mod tests {
  use crate::tools::parse;
  use crate::tools::parse_mul;

  #[test]
  fn test_parse_single() {
    assert_eq!(parse::<u8>("0"), Ok(0u8));
    assert_eq!(parse::<i32>("1"), Ok(1i32));
    assert!(parse::<u8>("-1").is_err());
    assert!(parse::<i32>("a").is_err());
    assert!(parse::<i32>("-").is_err());
  }

  #[test]
  fn test_parse_multiple() {
    assert_eq!(parse_mul::<u8>("7 11 8 6 3 8 9", None), Ok(vec![7u8, 11u8, 8u8, 6u8, 3u8, 8u8, 9u8]));
    assert_eq!(parse_mul::<u8>("7,11,8,6,3,8,9", Some(",".into())), Ok(vec![7u8, 11u8, 8u8, 6u8, 3u8, 8u8, 9u8]));
    assert_eq!(parse_mul::<u8>("7 11 8 6     9", None), Ok(vec![7u8, 11u8, 8u8, 6u8, 9u8]));
  }

  use crate::algo_tasks::leetcode13::roman_to_int;

  #[test]
  fn test_roman_to_int() {
    assert_eq!(roman_to_int("MMXXIII".into()), Ok(2023));
  }

  use crate::algo_tasks::leetcode412::fizz_buzz;

  #[test]
  fn test_fizz_buzz() {
    assert_eq!(fizz_buzz(17)[14], "FizzBuzz");
  }

  use crate::structs::single_linked_list::{ListNode, make_single_linked_list};
  use crate::algo_tasks::leetcode876::middle_node;

  #[test]
  fn test_middle_node() {
    assert_eq!(
      middle_node(make_single_linked_list(&vec![1, 2, 3, 4, 5])),
      Some(Box::new(ListNode {
        val: 3,
        next: Some(Box::new(ListNode {
          val: 4,
          next: Some(Box::new(ListNode {
            val: 5,
            next: None
          }))
        }))
      }))
    );
    assert_eq!(
      middle_node(make_single_linked_list(&vec![1, 2, 3, 4, 5, 6])),
      Some(Box::new(ListNode {
        val: 4,
        next: Some(Box::new(ListNode {
          val: 5,
          next: Some(Box::new(ListNode {
            val: 6,
            next: None
          }))
        }))
      }))
    );
  }
  
  use crate::structs::matrix::Matrix;
  use crate::algo_tasks::leetcode1337::k_weakest_rows;
  
  #[test]
  fn test_k_weakest_rows() {
    assert_eq!(k_weakest_rows(
      Matrix::<i32>::from(
        vec![vec![1,1,0,0,0],
             vec![1,1,1,1,0],
             vec![1,0,0,0,0],
             vec![1,1,0,0,0],
             vec![1,1,1,1,1]],
      ).unwrap(), 3
    ),
    vec![2,0,3]);
    assert_eq!(k_weakest_rows(
      Matrix::<i32>::from(
        vec![vec![1,0,0,0],
             vec![1,1,1,1],
             vec![1,0,0,0],
             vec![1,0,0,0]],
      ).unwrap(), 2
    ),
    vec![0,2]);
  }

  use crate::algo_tasks::leetcode1342::number_of_steps;

  #[test]
  fn test_number_of_steps_to_reduce() {
    assert_eq!(number_of_steps(14), 6);
    assert_eq!(number_of_steps(8), 4);
    assert_eq!(number_of_steps(123), 12);
  }

  use crate::algo_tasks::leetcode1672::maximum_wealth;

  #[test]
  fn test_maximum_wealth() {
    assert_eq!(maximum_wealth(Matrix::from(
      vec![
        vec![1, 2, 3],
        vec![3, 2, 1]
      ]).unwrap()), 6);
    assert_eq!(maximum_wealth(Matrix::from(
      vec![
        vec![1,5],
        vec![7,3],
        vec![3,5]
      ]).unwrap()), 10);
    assert_eq!(maximum_wealth(Matrix::from(
      vec![
        vec![2,8,7],
        vec![7,1,3],
        vec![1,9,5]
      ]).unwrap()), 17);
  }

  use crate::structs::binary_search_tree::BinarySearchTree;

  #[test]
  fn test_bst() {
    let mut bst1 = BinarySearchTree::<i32>::new();
    bst1.insert(5).unwrap();
    bst1.insert(4).unwrap();
    bst1.insert(6).unwrap();
    let mut bst2 = BinarySearchTree::<i32>::new();
    bst2.insert(5).unwrap();
    bst2.insert(6).unwrap();
    bst2.insert(4).unwrap();
    assert_eq!(bst1, bst2);
    let mut bst3 = BinarySearchTree::<i32>::new();
    bst3.insert(4).unwrap();
    bst3.insert(5).unwrap();
    bst3.insert(6).unwrap();
    assert_ne!(bst1, bst3);
  }
}
