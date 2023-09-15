/// Однонаправленный список.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

// impl ListNode {
//   /// Создаёт узел списка.
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }

/// Создаёт список из вектора чисел.
pub fn make_single_linked_list(numbers: &Vec<i32>) -> Option<Box<ListNode>> {
  numbers
    .into_iter()
    .rev()
    .fold(None, |next, val| {
      Some(Box::new(ListNode {
        val: *val,
        next,
      }))
    })
}
