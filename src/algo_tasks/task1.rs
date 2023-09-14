use crate::tools::{MResult, println, read, read_mul};

/// Простой сток. Мы идём по списку назад и считаем, сколько значений меньше текущего.
///
/// Сложность: O(n^2)
fn simple_stock_span(quotes: &Vec<u8>, debug: bool) -> Vec<u8> {
  if debug { println("\tsimple_stock_span running..."); }
  let mut spans = Vec::with_capacity(quotes.len());
  for i in 0..quotes.len() {
    if debug { println(&format!("\ti = {}:", i)); }
    let mut k: usize = 1;
    let mut span_end = false;
    while i.checked_sub(k).is_some() && !span_end {
      if debug { println(&format!("\t\ti - k = {}", i.checked_sub(k).unwrap())); }
      if debug { println(&format!("\t\t{} <= {} comparison: k += 1 or break", quotes[i.checked_sub(k).unwrap()], quotes[i])); }
      if quotes[i.checked_sub(k).unwrap()] <= quotes[i] { k += 1; }
      else { span_end = true; }
      if debug { println(&format!("\t\tk is now {}", k)); }
    }
    if debug { println(&format!("\t\tfor i = {} pushing k = {}", i, k)); }
    spans.push(k as u8);
  }
  spans
}

/// Улучшение верхнего алгоритма при помощи стека.
/// Мы кладём в стек номер дня, чтобы потом в цикле узнать, больше ли значение акций тем днём, что наверху стека, чем в текущем дне.
/// Если больше, мы выходим из цикла; если меньше, мы вынимаем последний элемент.
/// Если после цикла стек у нас пуст, это значит, что сегодня курс акций выше, чем за всё время.
/// Если не пуст, то мы кладём разницу между номерами текущего дня и дня, когда курс акций был выше, чем за сегодня.
/// Опять-таки, не забываем после этого положить в стек номер текущего дня.
///
/// Сложность: O(n)
fn stack_stock_span(quotes: &Vec<u8>, debug: bool) -> Vec<u8> {
  if debug { println("\tstack_stock_span running..."); }
  let mut spans = Vec::with_capacity(quotes.len());
  let mut stack = Vec::<usize>::with_capacity(quotes.len());
  stack.push(0usize);
  if debug { println("\tin stack: [0]"); }
  for i in 0..quotes.len() {
    if debug { println(&format!("\ti = {}:", i)); }
    if debug { println("\t\twhile stack is not empty:"); }
    while !stack.is_empty() {
      let last_from_stack: &usize = stack.last().unwrap();
      if debug { println(&format!("\t\t\t{} > {} comparison: break or pop", quotes[*last_from_stack], quotes[i])); }
      if quotes[*last_from_stack] > quotes[i] {
        break
      }
      stack.pop();
    }
    if stack.is_empty() {
      if debug { println(&format!("\t\tstack is empty, pushing to spans {}", (i as u8) + 1)); }
      spans.push((i as u8) + 1);
    }
    else {
      let last_from_stack: &usize = stack.last().unwrap();
      if debug { println(&format!("\t\tstack is not empty, pushing to spans {}", (i as u8) - (*last_from_stack as u8))); }
      spans.push((i as u8) - (*last_from_stack as u8));
    }
    if debug { println(&format!("\t\tpushing to stack {}", i)); }
    stack.push(i);
    if debug { println(&format!("\tin stack: {:?}", &stack)); }
  }
  spans
}

pub fn simple_stock_span_task() -> MResult {
  println("Введите числа, соответствующие ценам акций с первого по n-ный дни: ");
  let quotes: Vec<u8> = read_mul::<u8>(None, None)?;
  println("Вывести данные отладки? [y/n]");
  let opt: String = read::<String>(None)?;
  let simple_stock_res: Vec<u8>;
  let stack_stock_res: Vec<u8>;
  if opt == "y" {
    simple_stock_res = simple_stock_span(&quotes, true);
    stack_stock_res = stack_stock_span(&quotes, true);
  } else {
    simple_stock_res = simple_stock_span(&quotes, false);
    stack_stock_res = stack_stock_span(&quotes, false);
  }
  println(&format!("Обычный (наивный) алгоритм: {:?}", simple_stock_res));
  println(&format!("Алгоритм с использованием стеков: {:?}", stack_stock_res));
  Ok(())
}
