//! Однонаправленный список элементов `T`.

pub trait NodeValue = PartialEq + Clone;

/// Однонаправленный список.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode<T: NodeValue> {
  pub val: T,
  pub next: Option<Box<ListNode<T>>>
}

impl<T: NodeValue> ListNode<T> {
  /// Создаёт узел списка.
  #[inline]
  #[allow(dead_code)]
  pub fn new(val: T) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

/// Создаёт список из вектора чисел.
pub fn make_single_linked_list<T: NodeValue>(numbers: &Vec<T>) -> Option<Box<ListNode<T>>> {
  numbers
    .into_iter()
    .rev()
    .fold(None, |next, val| {
      Some(Box::new(ListNode {
        val: val.clone(),
        next,
      }))
    })
}
