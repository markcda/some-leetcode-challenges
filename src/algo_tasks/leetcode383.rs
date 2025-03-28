use crate::tools::{MResult, read, println};

/// Функция сообщает, можно ли собрать из букв в "магазине" заметку.
fn can_construct(ransom_note: String, magazine: String) -> bool {
  let mut vec = Vec::with_capacity(256);
  for _ in 0..256 { vec.push(0); }
  for i in magazine.chars() {
    vec[i as usize] += 1;
  }
  for i in ransom_note.chars() {
      if vec[i as usize] == 0 { return false; }
    vec[i as usize] -= 1;
  }
  true
}

pub fn leetcode383_task() -> MResult {
  let ransom_note: String = read(Some("Введите заметку, которую вы хотели бы составить:"))?;
  let magazine: String = read(Some("Введите магазин доступных букв:"))?;
  match can_construct(ransom_note, magazine) {
    true => println("Да, можно составить заметку!"),
    false => println("Нет, заметку составить нельзя."),
  }
  Ok(())
}
