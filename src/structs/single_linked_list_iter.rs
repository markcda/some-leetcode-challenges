use crate::structs::single_linked_list::{ListNode, NodeValue};

/// Итератор по однонаправленному списку.
pub struct IterListNode<'a, T: NodeValue> {
  head_ref: Option<&'a ListNode<T>>,
}

impl<'a, T: NodeValue> IterListNode<'a, T> {
  /// Создаёт итератор.
  pub fn new(head_ref: Option<&'a ListNode<T>>) -> Self {
    Self { head_ref }
  }

  /// Функция, определяющая, является ли текущее значение `self.head_ref.unwrap().val` идентичным значению `node.next()...next().val`.
  /// После проверки текущее значение итератора смещается, а поскольку функция рекурсивна и сразу после смещения поднимается на состояние выше,
  ///   то мы получаем де-факто двусвязную проверку, в ходе которой состояние предыдущей выбранной для проверки ноды хранится в стеке рекурсивных
  ///   вызовов. На самом деле, это не лучшее решение. Посмотреть другие решения: https://leetcode.com/problems/palindrome-linked-list/solutions/
  pub fn is_palindrome(&mut self, node: Option<&ListNode<T>>) -> bool {
    if let Some(some_node) = node {
      if !self.is_palindrome(some_node.next.as_deref()) {
        return false;
      }
      if some_node.val != self.head_ref.unwrap().val {
        return false;
      }
      self.head_ref = self.head_ref.unwrap().next.as_deref();
    }
    true
  }
}
