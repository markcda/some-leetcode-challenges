use std::str::FromStr;
use std::ops::{Index, IndexMut};

use crate::tools::{TResult, parse_mul};

pub trait MatrixElement: FromStr + Default + Clone {}
impl<T> MatrixElement for T where T: FromStr + Default + Clone {}

/// Матрица с размерами.
#[derive(Debug, Clone)]
pub struct Matrix<T: MatrixElement> {
  vals: Vec<Vec<T>>,
  pub size: (usize, usize),
}

/// Трейты для сборки матриц из итераторов.
trait TryFromRaw<T: MatrixElement> {
  fn try_make_matrix(self) -> Option<Matrix<T>>;
}

impl<T: MatrixElement> TryFromRaw<T> for Vec<Vec<T>> {
  /// Функция собирает матрицу тогда и только тогда, когда длины всех векторов совпадают.
  fn try_make_matrix(self) -> Option<Matrix<T>> {
    let mut size: Option<(usize, usize)> = None;
    for row in self.iter() {
      match size {
        None => {
          size = Some((row.len(), 1));
        },
        Some((row_len, rows_total)) => {
          if row.len() != row_len { return None }
          size = Some((row_len, rows_total + 1));
        },
      }
    }
    Some(Matrix { vals: self, size: size.unwrap() })
  }
}

trait CollectRowsFromResults<T: MatrixElement> {
  fn collect_or_none(&mut self) -> Option<Matrix<T>>;
}

impl<'a, T, I> CollectRowsFromResults<T> for I
where
  T: MatrixElement,
  I: Iterator<Item = TResult<Vec<T>>>,
{
  /// Функция собирает матрицу тогда и только тогда, если каждый элемент итератора является `Result<Vec<T>>::Ok()`,
  /// и длины всех векторов совпадают.
  fn collect_or_none(&mut self) -> Option<Matrix<T>> {
    let mut vals: Vec<Vec<T>> = Vec::new();
    let mut size: Option<(usize, usize)> = None;
    while let Some(val) = self.next() {
      match val {
        Ok(v) => match size {
          None => {
            size = Some((v.len(), 1));
            vals.push(v);
          },
          Some((row_len, rows_total)) => {
            if v.len() != row_len { return None }
            size = Some((row_len, rows_total + 1));
            vals.push(v);
          },
        },
        Err(_) => return None,
      }
    };
    Some(Matrix { vals, size: size.unwrap() })
  }
}

/// Индексирование для матрицы. Указывается номер ряда (`y`-координата), возвращается ряд.
impl<T: MatrixElement> Index<usize> for Matrix<T> {
    type Output = Vec<T>;
    fn index(&self, i: usize) -> &Vec<T> {
        &self.vals[i]
    }
}

impl<T: MatrixElement> IndexMut<usize> for Matrix<T> {
    fn index_mut(&mut self, i: usize) -> &mut Vec<T> {
        &mut self.vals[i]
    }
}

/// Методы матрицы.
impl<T: MatrixElement> Matrix<T> {
  /// Создаёт пустую матрицу с заданной ёмкостью.
  #[allow(dead_code)]
  pub fn with_capacity(size: (usize, usize)) -> Self {
    let mut vals: Vec<Vec<T>> = Vec::with_capacity(size.1);
    for _ in 0..size.1 {
      let mut v: Vec<T> = Vec::with_capacity(size.0);
      for _ in 0..size.0 { v.push(Default::default()); }
      vals.push(v);
    }
    Matrix {
      vals,
      size,
    }
  }
  
  /// Создаёт матрицу из вектора векторов.
  #[allow(dead_code)]
  pub fn from(raw_matrix: Vec<Vec<T>>) -> TResult<Self> {
    raw_matrix
      .try_make_matrix()
      .ok_or("Не удалось преобразовать набор векторов значений в матрицу")
  }
  
  /// Собирает матрицу из набора строк, которые перед этим парсятся в наборы значений `T`.
  pub fn parse_from_lines(strs: Vec<String>) -> TResult<Self> {
    strs
      .iter()
      .map(|s| parse_mul::<T>(s, None))
      .collect_or_none()
      .ok_or("Не удалось преобразовать набор строк в матрицу")
  }
  
  /// Возвращает количество строк матрицы.
  pub fn len(&self) -> usize {
    return self.size.1;
  }
}
