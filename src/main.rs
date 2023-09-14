mod algo_tasks;
mod structs;
mod tests;
mod tools;

use crate::algo_tasks::task1::simple_stock_span_task;
use crate::algo_tasks::leetcode13::leetcode13_task;
use crate::algo_tasks::leetcode234::leetcode234_task;
use crate::tools::{MResult, read_opt};

fn main() -> MResult {
    println!("Привет! Что хочешь сделать?");
    loop {
        println!("1. Простой алгоритм для задачи о разнице курсов акций");
        println!("2. Римские числа в арабские");
        println!("3. Списки-палиндромы");
        match read_opt()? {
            1u8 => simple_stock_span_task()?,
            2u8 => leetcode13_task()?,
            3u8 => leetcode234_task()?,
            _ => break,
        };
        println!("\nВыбери задачу:");
    }
    Ok(())
}
