use crate::structs::single_linked_list::{ListNode, NodeValue, make_single_linked_list};
use crate::tools::{MResult, read_mul, println};

/// Возвращает середину односвязного списка. Если в списке чётное число элементов, возвращает второй элемент середины.
/// Использует два указателя по списку: быстрый и медленный. Быстрый делает два шага по списку на каждый шаг медленного.
pub fn middle_node<T: NodeValue>(head: Option<Box<ListNode<T>>>) -> Option<Box<ListNode<T>>> {
  if head.is_none() { return None; }
  let mut slow = head.as_ref();
  let mut fast = head.as_ref();
  loop {
    if fast.unwrap().next.is_some() && fast.unwrap().next.as_ref().unwrap().next.is_some() {
      fast = fast.unwrap().next.as_ref().unwrap().next.as_ref();
      slow = slow.unwrap().next.as_ref();
    } else if fast.unwrap().next.is_some() {
      slow = slow.unwrap().next.as_ref();
      return slow.cloned();
    } else {
      return slow.cloned();
    }
  }
}

pub fn leetcode876_task() -> MResult {
  let numbers: Vec<i32> = read_mul(Some("Введите числа через пробел: "), None)?;
  let head: Option<Box<ListNode<i32>>> = make_single_linked_list(&numbers);
  match middle_node(head) {
    Some(result_node) => println(&format!("Середина списка: {}, полный вывод: {:?}", result_node.val, result_node)),
    None => return Err("Не удалось найти середину списка"),
  }
  Ok(())
}

#[cfg(test)]
mod tests {
  #[test]
  fn test_middle_node() {
    use crate::structs::single_linked_list::{ListNode, make_single_linked_list};
    use crate::algo_tasks::leetcode876::middle_node;
    
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
}
