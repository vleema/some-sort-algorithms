mod algorithms;

use algorithms::*;
use rand::Rng;
use std::{any::Any, io::Write, time::Instant};

const TEN: usize = 10;
const ENTRY_SIZE: usize = 1000;

type SortFn<T> = fn(&mut [T]);

fn main() {
  let sort_functions = Vec::from([
    ("SELECTION SORT", selection_sort as SortFn<i32>),
    ("BUBBLE SORT", bubble_sort),
    ("INSERTION SORT", insertion_sort),
    ("QUICK SORT", quick_sort),
  ]);

  let title = "Performance test for sort algorthms";
  println!("{}\n{}\n", title, "=".repeat(title.len()));

  for n in 1..=4 {
    run_entry(&sort_functions, TEN.pow(n));
  }
  // run_entry(&sort_functions, ENTRY_SIZE);
}

fn run_entry(functions: &Vec<(&str, SortFn<i32>)>, entry_size: usize) {
  let gen_start = Instant::now();
  print!("Generating list with {} entries", entry_size);
  let list = gen_random_list(entry_size);
  println!(" ... ({:?})\n", gen_start.elapsed());

  for (func_name, func) in functions {
    println!("{}:", func_name);
    print!("  >>> Cloning ... ");
    std::io::stdout().flush().unwrap();
    let clone_start = Instant::now();
    let mut list_copy = list.clone();
    println!("finished in {:?}", clone_start.elapsed());

    print!("  >>> Running ... ");
    std::io::stdout().flush().unwrap();
    let run_start = Instant::now();
    func(&mut list_copy);
    println!("finished in {:?}\n", run_start.elapsed())
  }
  println!("{}\n", "-".repeat(50));
}

fn gen_random_list(size: usize) -> Vec<i32> {
  let mut rng = rand::thread_rng();
  let mut arr = Vec::with_capacity(size);
  for _ in 0..size {
    arr.push(rng.gen::<i32>());
  }
  arr
}
