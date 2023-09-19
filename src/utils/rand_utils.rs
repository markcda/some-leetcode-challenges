use std::slice::Iter;

pub fn find_index_of_random_range(probability_value: f64, container_len: usize, mut probabilities_iter: Iter<'_, f64>) -> usize {
  let mut i: usize = 0;
  let mut left_side: f64 = 0.0f64;
  while let Some(next_probability) = probabilities_iter.next() {
    if probability_value >= left_side && probability_value <= left_side + next_probability { break }
    left_side += next_probability;
    if i + 1 < container_len { i += 1 } else { break }
  }
  i
}