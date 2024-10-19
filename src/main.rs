mod algorithms;

use algorithms::*;
use std::time::Instant;

const PADDING_SIZE: usize = 12;

fn main() {
  {
    let mut arr = [2, 42, 39, 10, 76, 432, 42, 97];
    header("selection sort");
    body(selection_sort, &mut arr);
  }
  {
    let mut arr = [35, 12, 2, 90, 42, 78, 9, 10];
    header("bubble sort");
    body(bubble_sort, &mut arr);
  }
  {
    let mut arr = [365, 72, 2, 90, 42, 78, 9, 10];
    header("insertion sort");
    body(insertion_sort, &mut arr);
  }
  {
    let mut arr = [98, 33, 24, 69, 11, 2, 42, 7];
    header("quick sort");
    body(quick_sort, &mut arr);
  }
  // {
  //     let mut arr = [35, 12, 2, 90, 42, 78, 9, 10];
  //     header("merge sort");
  //     body(merge_sort, &mut arr);
  // }
}

fn header(test: &str) {
  let padding = "=".repeat(PADDING_SIZE);
  println!("{}[ {} ]{}", padding, test, padding)
}

fn body<T: std::fmt::Debug>(f: fn(&mut [T]), arr: &mut [T]) {
  println!(">>> Array before sorting: {:?}", arr);
  let start = Instant::now();
  f(arr);
  let duration = start.elapsed();
  println!(">>> Array after sorting:  {:?}", arr);
  println!(" -- Time taken: {:?}\n", duration);
}
