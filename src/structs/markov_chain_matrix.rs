use rand::Rng;

use crate::structs::matrix::Matrix;
use crate::tools::TResult;
use crate::utils::rand_utils::find_index_of_random_range;

pub struct MarkovChainMatrix<T: Clone> {
  compiled: bool,
  values: Vec<T>,
  probabilities: Matrix<f64>,
  pub current_state: usize,
  pub frequences: Vec<u16>,
}

impl<T: Clone> MarkovChainMatrix<T> {
  /// WARNING: нет проверки на валидность суммы вероятностей по строкам и столбцам матрицы
  pub fn from(values: Vec<T>, probabilities: Matrix<f64>, current_state: usize) -> TResult<Self> {
    if values.len() != probabilities.len() { return Err("Размеры матрицы вероятностей и элементов не совпадают") }
    if probabilities.len() == 0 { return Err("Матрица вероятностей пуста") }
    if probabilities.len() != probabilities[0].len() { return Err("Матрица не квадратная") }
    if current_state >= values.len() { return Err("Текущее состояние не входит в отрезок [0,1]") }
    let mut frequences = Vec::with_capacity(values.len());
    for _ in 0..values.len() { frequences.push(0); }
    Ok(MarkovChainMatrix {
      values,
      probabilities,
      frequences,
      current_state,
      compiled: true,
    })
  }
  
  #[inline]
  fn check_compiled(&self) { if !self.compiled { panic!("Цепь не скомпилирована.") } }
  
  pub fn len(&self) -> usize {
    self.check_compiled();
    self.values.len()
  }
  
  pub fn values(&self) -> &Vec<T> {
    self.check_compiled();
    &self.values
  }
  
  pub fn generate_value(&mut self) -> &T {
    self.check_compiled();
    let mut rng = rand::thread_rng();
    let probability_value: f64 = 1.0f64 - rng.gen::<f64>();
    let i = find_index_of_random_range(probability_value, self.len(), self.probabilities[self.current_state].iter());
    self.current_state = i;
    self.frequences[i] += 1; 
    &self.values[i]
  }
}