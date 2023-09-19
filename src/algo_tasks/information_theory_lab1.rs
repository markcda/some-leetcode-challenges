use crate::structs::random_ensemble::RandomEnsemble;
use crate::structs::markov_chain_matrix::MarkovChainMatrix;
use crate::structs::matrix::Matrix;
use crate::tools::{MResult, read, read_matrix, print, println};

pub fn discrete_random_value_modeling_example() -> MResult {
  let mut ensemble = RandomEnsemble::from(vec![
    ("x1".to_owned(), 0.2f64),
    ("x2".to_owned(), 0.3f64),
    ("x3".to_owned(), 0.45f64),
    ("x4".to_owned(), 0.05f64),
  ])?;
  let amount_of_values: u32 = read(Some("Введите число генерируемых значений:"))?;
  print("\t")?;
  for _ in 0..amount_of_values {
    print(ensemble.generate_value())?;
  }
  println!();
  println("Частоты появления значений:");
  for i in 0..ensemble.len() {
    println(&format!("{} - {}", ensemble.values()[i], ensemble.frequences[i]));
  }
  Ok(())
}

pub fn markov_chain_modeling_example() -> MResult {
  let mut chain = MarkovChainMatrix::from(
    vec![
      "s1".to_owned(),
      "s2".to_owned(),
      "s3".to_owned(),
      "s4".to_owned(),
    ],
    Matrix::from(vec![
      vec![0.11f64, 0.36f64, 0.19f64, 0.34f64],
      vec![0.27f64, 0f64,    0.45f64, 0.28f64],
      vec![0.35f64, 0.29f64, 0.05f64, 0.31f64],
      vec![0.23f64, 0.48f64, 0.29f64, 0f64   ],
    ])?,
    0,
  )?;
  let amount_of_values: u32 = read(Some("Введите число генерируемых значений:"))?;
  print("\t")?;
  for _ in 0..amount_of_values {
    print(chain.generate_value())?;
  }
  println!();
  println("Частоты появления значений:");
  for i in 0..chain.len() {
    println(&format!("{} - {}", chain.values()[i], chain.frequences[i]));
  }
  Ok(())
}

pub fn discrete_random_value_modeling() -> MResult {
  let capacity: usize = read(Some("Введите число вариантов данных:"))?;
  let mut ensemble = RandomEnsemble::with_capacity(capacity);
  let mut probability: f64;
  for i in 0..capacity {
    probability = read(Some(&format!("Введите вероятность №{}:", i + 1)))?;
    ensemble.insert(format!("p{}", i + 1), probability);
  }
  ensemble.compile()?;
  let amount_of_values: u32 = read(Some("Введите число генерируемых значений:"))?;
  print("\t")?;
  for _ in 0..amount_of_values {
    print(ensemble.generate_value())?;
  }
  println!();
  println("Частоты появления значений:");
  for i in 0..ensemble.len() {
    println(&format!("{} - {}", ensemble.values()[i], ensemble.frequences[i]));
  }
  Ok(())
}

pub fn markov_chain_modeling() -> MResult {
  let capacity: usize = read(Some("Введите число вариантов данных:"))?;
  let values: Vec<String> = (0..capacity).map(|i| format!("s{}", i + 1)).collect();
  println("Введите матрицу вероятностей, после ввода пропустите одну строку:");
  let probabilities = read_matrix()?;
  let mut chain = MarkovChainMatrix::from(
    values,
    probabilities,
    0,
  )?;
  let amount_of_values: u32 = read(Some("Введите число генерируемых значений:"))?;
  print("\t")?;
  for _ in 0..amount_of_values {
    print(chain.generate_value())?;
  }
  println!();
  println("Частоты появления значений:");
  for i in 0..chain.len() {
    println(&format!("{} - {}", chain.values()[i], chain.frequences[i]));
  }
  Ok(())
}