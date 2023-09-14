use crate::structs::single_linked_list::ListNode;
use crate::structs::single_linked_list_iter::IterListNode;

use crate::tools::{MResult, println, read_mul};

/// Функция, определяющая, является ли односвязный список палиндромом.
fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
  IterListNode::new(head.as_deref()).is_palindrome(head.as_deref())
}

pub fn leetcode234_task() -> MResult {
  let numbers: Vec<i32> = read_mul(Some("Введите числа через пробел: ".into()), None)?;
  let head: Option<Box<ListNode>> = numbers.into_iter().rev().fold(None, |next, val| {
    Some(Box::new(ListNode {
      val,
      next,
    }))
  });
  match is_palindrome(head) {
    true => println("Данный список является палиндромом."),
    false => println("Данный список - не палиндром."),
  }
  Ok(())
}
