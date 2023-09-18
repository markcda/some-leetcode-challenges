use rand::Rng;

use crate::structs::random_ensemble::RandomEnsemble;
use crate::tools::{MResult, read, print, println};

fn generate_value<T: Clone>(ensemble: &mut RandomEnsemble<T>) -> &T {
  let mut rng = rand::thread_rng();
  let probability_value: f64 = 1.0f64 - rng.gen::<f64>();
  let mut i: usize = 0;
  let mut left_side: f64 = 0.0f64;
  let mut probability_iter = ensemble.probabilities_iter();
  while let Some(next_probability) = probability_iter.next() {
    if probability_value >= left_side && probability_value <= left_side + ensemble.probabilities()[i] {
      ensemble.frequences[i] += 1;
      break
    }
    left_side += next_probability;
    if i + 1 < ensemble.len() { i += 1 } else { ensemble.frequences[i] += 1; break }
  }
  &ensemble.values()[i]
}

pub fn discrete_random_value_modeling() -> MResult {
  let mut ensemble: RandomEnsemble<String> = RandomEnsemble::from(vec![
    ("x1".into(), 0.2f64),
    ("x2".into(), 0.3f64),
    ("x3".into(), 0.45f64),
    ("x4".into(), 0.05f64),
  ])?;
  let amount_of_values: u32 = read(Some("Введите число генерируемых значений:"))?;
  print("\t")?;
  for _ in 0..amount_of_values {
    print(generate_value::<String>(&mut ensemble))?;
  }
  println!();
  println("Частоты появления значений:");
  for i in 0..ensemble.len() {
    println(&format!("{} - {}", ensemble.values()[i], ensemble.frequences[i]));
  }
  Ok(())
}