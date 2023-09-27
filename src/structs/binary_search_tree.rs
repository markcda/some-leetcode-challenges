//! Улучшенная реализация бинарного дерева от [nyxtom](https://youtu.be/yHi3q2Iiepc).
//! В общем случае стоит использовать `std::collections::BTreeSet`, которая имеет больший набор функций. 

use crate::tools::{MResult, TResult};

use std::collections::HashSet;

/// Сокращение для `Option<Box<BinarySearchTreeNode<T>>>` - по аналогии с `TResult<T> = Result<T, &'static str>` и `MResult = Result<(), &'static str>`
pub type MNode<T> = Option<Box<BSTNode<T>>>;

/// Дерево бинарного поиска.
#[derive(PartialEq, Eq, Debug)]
pub struct BinarySearchTree<T: PartialOrd> {
  pub root: MNode<T>,
}

impl<T: PartialOrd> BinarySearchTree<T> {
  /// Создаёт пустое дерево.
  #[allow(dead_code)]
  pub fn new() -> Self {
    BinarySearchTree {
      root: None,
    }
  }
  
  /// Вставляет новый элемент типа `T` в бинарное дерево.
  ///
  /// См. также: `BSTNode<T>::insert(value: T)`
  #[allow(dead_code)]
  pub fn insert(&mut self, value: T) -> MResult {
    match &mut self.root {
      None => self.root = BSTNode::new(value).into(),
      Some(node) => node.insert_faster(value)?,
    }
    Ok(())
  }
  
  /// Создаёт дерево из вектора.
  #[allow(dead_code)]
  pub fn from_vec(values: Vec<T>) -> TResult<Self> {
    let mut tree = BinarySearchTree::<T>::new();
    for value in values {
      tree.insert(value)?;
    }
    Ok(tree)
  }
  
  /// Создаёт дерево из набора `std::collections::HashSet`.
  #[allow(dead_code)]
  pub fn from_hashset(values: HashSet<T>) -> TResult<Self> {
    let mut tree = BinarySearchTree::<T>::new();
    for value in values {
      tree.insert(value)?;
    }
    Ok(tree)
  }
}

/// Узел дерева бинарного поиска.
///
/// Содержит значение `T`, а также указатели на левого и правого детей.
#[derive(PartialEq, Eq, Debug)]
pub struct BSTNode<T: PartialOrd> {
  value: T,
  left: MNode<T>,
  right: MNode<T>,
}

impl<T: PartialOrd> BSTNode<T> {
  /// Создаёт узел ДБП со значением `T` и без детей.ч
  pub fn new(value: T) -> Self {
    BSTNode {
      value,
      left: None,
      right: None,
    }
  }

  /// Конструкция дерева такова, что элементы в нём упорядочены по возрастанию.
  ///
  /// Для того, чтобы добавить элемент, нужно узнать, больше ли он текущего элемента или нет.
  ///     Если вставляемый элемент больше текущего, он будет находиться справа от текущего - в правой ветви дерева.
  ///     Если не больше, то слева - в левой ветви дерева.
  ///
  /// Соответственно, если правая или левая ветвь пустая, то создаётся новый узел дерева с данным значением; если не пустая, то мы
  ///     рекурсивно вызываем функцию insert для выбранной ветви.
  ///
  /// _Размышления_: можно написать реализацию на основе динамического программирования с выбором мутабельного указателя на текущую ветвь дерева.
  pub fn insert(&mut self, value: T) -> MResult {
    if value > self.value {
      match &mut self.right {
        None => self.right = BSTNode::new(value).into(),
        Some(n) => n.insert(value)?,
      }
    } else if value < self.value {
      match &mut self.left {
        None => self.left = BSTNode::new(value).into(),
        Some(n) => n.insert(value)?,
      }
    }
    Ok(())
  }

  /// Реализация вставки на основе динамического программирования с выбором мутабельного указателя.
  ///
  /// Q: Почему мы используем `Some(_)` вместо `Some(foo)`?
  /// A: Потому что мы получим конфликт двух мутабельных ссылок на `node_ptr.<left | right>`.
  ///     Поэтому мы опускаем паттерн и получаем следующую ноду для сравнения через полный запрос:
  ///     `node_ptr = node_ptr.<left | right>.as_mut().unwrap().as_mut()` - мы можем делать `unwrap()`, так как
  ///     только что получили `Some(_)`.
  pub fn insert_faster(&mut self, value: T) -> MResult {
    let mut node_ptr = self;
    loop {
      if value > node_ptr.value {
        match node_ptr.right {
          None => { node_ptr.right = BSTNode::new(value).into(); break },
          Some(_) => node_ptr = node_ptr.right.as_mut().unwrap().as_mut(),
        }
      } else if value < node_ptr.value {
        match node_ptr.left {
          None => { node_ptr.left = BSTNode::new(value).into(); break },
          Some(_) => node_ptr = node_ptr.left.as_mut().unwrap().as_mut(),
        }
      } else {
        return Err("Найден идентичный элемент, вставить его невозможно.");
      }
    }
    Ok(())
  }
}

impl<T: PartialOrd> From<BSTNode<T>> for MNode<T> {
  /// Создаёт обёртку вокруг узла ДБП.
  fn from(node: BSTNode<T>) -> Self {
    Option::Some(Box::new(node))
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn test_bst() {
    use crate::structs::binary_search_tree::BinarySearchTree;
    
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
