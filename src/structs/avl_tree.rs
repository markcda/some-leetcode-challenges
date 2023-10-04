//! Реализация АВЛ-дерева по [референсу](https://habr.com/ru/articles/150732/).

use std::cmp::Ordering;

pub trait NodeValue = PartialOrd + std::fmt::Debug;

/// Упрощённый тип - замена указателя.
pub type ANode<T> = Option<Box<AVLNode<T>>>;

/// Структура узла.
///
/// Поскольку АВЛ-дерево является сбалансированным, то его высота `height < 1.44 * log_2(n + 2)`,
///     то есть при 10^9 ключей высота дерева не превысит 44 узла. Так что берём!
#[derive(Debug)]
pub struct AVLNode<T: NodeValue> {
  pub key: T,
  pub height: u8,
  left: ANode<T>,
  right: ANode<T>,
}

/// Воплотим некоторые методы узла над указателем.
pub trait AVLPtrOps<T: NodeValue> {
  fn height(&self) -> u8;
  fn insert(self, k: T) -> Self;
  fn remove(self, k: T) -> Self;
  fn find(&self, k: T) -> bool;
  fn count(&self) -> u128;
}

impl<T: NodeValue> AVLPtrOps<T> for ANode<T> {
  fn height(&self) -> u8 {
    match &self {
      None => 0,
      Some(node) => node.height,
    }
  }
  
  fn insert(self, k: T) -> Self {
    match self {
      None => AVLNode::new(k),
      Some(node) => node.insert(k),
    }
  }
  
  fn remove(self, k: T) -> Self {
    match self {
      None => self,
      Some(node) => node.remove(k),
    }
  }
  
  fn find(&self, k: T) -> bool {
    match self {
      None => false,
      Some(node) => node.find(k),
    }
  }
  
  fn count(&self) -> u128 {
    match self {
      None => 0,
      Some(node) => node.count(),
    }
  }
}

/// Функции над узлами дерева.
impl<T: NodeValue> AVLNode<T> {
  /// Создаёт новый узел в обёртке.
  pub fn new(key: T) -> ANode<T> {
    Some(Box::new(AVLNode {
      key,
      height: 1,
      left: None,
      right: None,
    }))
  }
  
  /// Вычисляет фактор балансировки. O(1)
  pub fn b_factor(&self) -> i16 {
    self.right.height() as i16 - self.left.height() as i16
  }
  
  /// Исправляет высоту дерева. O(1)
  pub fn fix_height(&mut self) {
    let hl = self.left.height();
    let hr = self.right.height();
    self.height = match hl.cmp(&hr) {
      Ordering::Greater => hl,
      Ordering::Less => hr,
      _ => hl,
    } + 1;
  }
  
  /// Делает поворот АВЛ-дерева вправо.
  ///
  /// Метод перемещает текущий узел в памяти. O(1)
  pub fn rotate_right(mut self) -> ANode<T> {
    let mut q = self.left.unwrap();
    self.left = q.right;
    q.right = Some(Box::new(self));
    q.right.as_mut().unwrap().fix_height();
    q.fix_height();
    Some(Box::new(*q))
  }
  
  /// Делает поворот АВЛ-дерева влево.
  ///
  /// Метод перемещает текущий узел в памяти. O(1)
  pub fn rotate_left(mut self) -> ANode<T> {
    let mut p = self.right.unwrap();
    self.right = p.left;
    p.left = Some(Box::new(self));
    p.left.as_mut().unwrap().fix_height();
    p.fix_height();
    Some(Box::new(*p))
  }
  
  /// Выполняет общую балансировку.
  ///
  /// При нормальном добавлении или удалении элементов фактор балансировки
  ///     не сможет превысить +-2, что позволяет текущему методу выполняться за O(1).
  pub fn balance(mut self) -> ANode<T> {
    self.fix_height();
    if self.b_factor() == 2 {
      if self.right.as_ref().unwrap().b_factor() < 0 {
        self.right = self.right.unwrap().rotate_right();
      }
      self.rotate_left()
    } else if self.b_factor() == -2 {
      if self.left.as_ref().unwrap().b_factor() > 0 {
        self.left = self.left.unwrap().rotate_left();
      }
      self.rotate_right()
    } else {
      Some(Box::new(self))
    }
  }
  
  /// Вставляет ключ `k: T` в дерево.
  ///
  /// Предварительно создавать узел не нужно.
  /// В процессе по необходимости выполняется балансировка. Количество действий балансировки -
  ///     O(height), - а это прям мало-мало!
  pub fn insert(mut self, k: T) -> ANode<T> {
    if k < self.key {
      if self.left.is_none() {
        self.left = AVLNode::new(k);
      } else {
        self.left = self.left.unwrap().insert(k);
      }
    } else {
      if self.right.is_none() {
        self.right = AVLNode::new(k);
      } else {
        self.right = self.right.unwrap().insert(k);
      }
    }
    self.balance()
  }
  
  /// Удаляет узел с минимальным ключом из текущего дерева.
  ///
  /// Нам нельзя перемещать узел в памяти, поэтому возвращаем изменённое дерево
  ///     и сам удалённый элемент.
  fn remove_min(mut self) -> (ANode<T>, ANode<T>) {
    if self.left.is_none() {
      if self.right.is_some() {
        let right = self.right.unwrap();
        self.right = None;
        (Some(Box::new(*right)), Some(Box::new(self)))
      } else {
        (None, Some(Box::new(self)))
      }
    } else {
      let (modified, min) = self.left.unwrap().remove_min();
      match modified {
        None => {
          self.left = None;
          (self.balance(), min)
        },
        Some(_) => {
          self.left = modified;
          (self.balance(), min)
        },
      }
    }
  }
  
  /// Удаляет узел с ключом `k: T` из дерева.
  pub fn remove(mut self, k: T) -> ANode<T> {
    if k < self.key {
      if self.left.is_none() {
        return None
      } else {
        self.left = self.left.unwrap().remove(k);
      }
    } else if k > self.key {
      if self.right.is_none() {
        return None
      } else {
        self.right = self.right.unwrap().remove(k);
      }
    } else { // k == self.key
      let right = match self.right.is_none() {
        true => return self.left,
        false => self.right.unwrap(),
      };
      let left = self.left;
      let (modified, min) = right.remove_min();
      let mut min = min.unwrap();
      min.right = modified;
      min.left = left;
      return min.balance();
    }
    self.balance()
  }
  
  /// Стандартная реализация поиска.
  pub fn find(&self, k: T) -> bool {
    if k < self.key { self.left.find(k) }
    else if k > self.key { self.right.find(k) }
    else { true }
  }
  
  /// Считаем число узлов в дереве.
  pub fn count(&self) -> u128 {
    let lc = self.left.count();
    let rc = self.right.count();
    lc + rc + 1
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn test_avl() {
    use crate::structs::avl_tree::{AVLNode, AVLPtrOps, ANode};
    
    let mut tree: ANode<i32> = AVLNode::new(16)
      .insert(17)
      .insert(15)
      .insert(1)
      .insert(20);
    
    assert!(tree.find(1));
    assert!(tree.find(15));
    assert!(tree.find(16));
    assert!(tree.find(17));
    assert!(tree.find(20));
    assert_eq!(tree.count(), 5);
    
    tree = tree.remove(17);
    assert!(tree.find(1));
    assert!(tree.find(15));
    assert!(tree.find(16));
    assert!(tree.find(20));
    assert_eq!(tree.count(), 4);
    
    tree = tree.insert(21).insert(22);
    assert!(tree.find(1));
    assert!(tree.find(15));
    assert!(tree.find(16));
    assert!(tree.find(20));
    assert!(tree.find(21));
    assert!(tree.find(22));
    assert_eq!(tree.count(), 6);
  }
}
