use crate::structs::single_linked_list::{ListNode, NodeValue, make_single_linked_list};
use crate::structs::single_linked_list_iter::IterListNode;

use crate::tools::{MResult, println, read_mul};

/// Функция, определяющая, является ли односвязный список палиндромом.
fn is_palindrome<T: NodeValue>(head: Option<Box<ListNode<T>>>) -> bool {
  IterListNode::new(head.as_deref()).is_palindrome(head.as_deref())
}

pub fn leetcode234_task() -> MResult {
  let numbers: Vec<i32> = read_mul(Some("Введите числа через пробел: "), None)?;
  let head: Option<Box<ListNode<i32>>> = make_single_linked_list(&numbers);
  match is_palindrome(head) {
    true => println("Данный список является палиндромом."),
    false => println("Данный список - не палиндром."),
  }
  Ok(())
}
