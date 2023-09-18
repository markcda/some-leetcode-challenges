use std::slice::{Iter, IterMut};
use crate::tools::{MResult, TResult};

/// Структура данных, хранящая величины `T` и вероятность их моделирования.
#[derive(Clone)]
pub struct RandomEnsemble<T: Clone> {
  compiled: bool,
  values: Vec<T>,
  probabilities: Vec<f64>,
  pub frequences: Vec<u16>,
}

impl<T: Clone> RandomEnsemble<T> {
  pub fn with_capacity(capacity: usize) -> Self {
    RandomEnsemble {
      compiled: false,
      values: Vec::with_capacity(capacity),
      probabilities: Vec::with_capacity(capacity),
      frequences: Vec::with_capacity(capacity),
    }
  }

  pub fn insert(&mut self, value: T, probability: f64) {
    self.values.push(value);
    self.probabilities.push(probability);
    self.frequences.push(0);
  }

  fn ensure_probabilities(&self) -> bool {
    let sum: f64 = self.probabilities.iter().sum();
    if (sum >= 0.9999) && (sum <= 1.0001) { true }
    else { false }
  }

  pub fn compile(&mut self) -> MResult {
    if self.values.capacity() != self.values.len() ||
      self.values.capacity() != self.frequences.len() { return Err("Ансамбль значений заполнен не до конца") }
    if !self.ensure_probabilities() { return Err("Погрешность сумм вероятностей составляет более одной десятитысячной") }
    self.compiled = true;
    Ok(())
  }

  pub fn from(vals: Vec<(T, f64)>) -> TResult<Self> {
    let mut ensemble = RandomEnsemble::with_capacity(vals.len());
    vals.iter().for_each(|el| ensemble.insert(el.0.clone(), el.1));
    ensemble.compile()?;
    Ok(ensemble)
  }

  #[inline]
  fn check_compiled(&self) { if !self.compiled { panic!("Ансамбль не скомпилирован.") } }

  pub fn len(&self) -> usize {
    self.check_compiled();
    self.values.len()
  }

  #[allow(dead_code)]
  pub fn values_iter(&self) -> Iter<'_, T> {
    self.check_compiled();
    self.values.iter()
  }

  #[allow(dead_code)]
  pub fn values_iter_mut(&mut self) -> IterMut<'_, T> {
    self.check_compiled();
    self.values.iter_mut()
  }
  pub fn probabilities_iter(&self) -> Iter<'_, f64> {
    self.check_compiled();
    self.probabilities.iter()
  }

  pub fn values(&self) -> &Vec<T> {
    self.check_compiled();
    &self.values
  }

  pub fn probabilities(&self) -> &Vec<f64> {
    self.check_compiled();
    &self.probabilities
  }
}