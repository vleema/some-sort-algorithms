/// Sorts a slice in place using the selection sort algorithm.
///
/// It iterates over the slice and for each element, finds the smallest item in the rest of the slice and swaps it with the current element.
///
/// # Arguments
///
/// * `arr` - A mutable slice of elements implementing `PartialOrd`.
///
/// # Example
///
/// ```
/// let mut arr = vec![5, 3, 2, 4, 1];
/// selection_sort(&mut arr);
/// assert_eq!(arr, vec![1, 2, 3, 4, 5]);
/// ```
pub fn selection_sort<T: PartialOrd>(arr: &mut [T]) {
  for i in 0..arr.len() - 1 {
    let mut min_index = i;
    for j in (i + 1)..arr.len() {
      if arr[j] < arr[min_index] {
        min_index = j;
      }
    }
    arr.swap(min_index, i);
  }
}

/// Sorts an array in place using the bubble sort algorithm
///
/// # Arguments
///
/// * `arr` - A mutable slice of elements that implement the `PartialOrd` trait
///
/// # Example
///
/// ```
/// let mut arr = [5, 3, 8, 4, 2];
/// bubble_sort(&mut arr);
/// assert_eq!(arr, [2, 3, 4, 5, 8]);
/// ```
pub fn bubble_sort<T: PartialOrd>(arr: &mut [T]) {
  let mut n = arr.len();
  let mut swapped = true;
  while swapped {
    swapped = false;
    for i in 1..n {
      if arr[i - 1] > arr[i] {
        arr.swap(i - 1, i);
        swapped = true;
      }
    }
    n -= 1;
  }
}

/// Sorts a slice in place using the insertion sort algorithm.
///
/// It iterates over the slice and maintains a sorted sublist in the lower positions of the list. Each new item is then "inserted" back into the previous sublist such that the sorted sublist is one item larger.
///
/// # Arguments
///
/// * `arr` - A mutable slice of elements implementing `PartialOrd`.
///
/// # Example
///
/// ```
/// let mut arr = vec![5, 3, 2, 4, 1];
/// insertion_sort(&mut arr);
/// assert_eq!(arr, vec![1, 2, 3, 4, 5]);
/// ```
pub fn insertion_sort<T: PartialOrd>(arr: &mut [T]) {
  for i in 1..arr.len() {
    let mut j = i;
    while j > 0 && arr[j] < arr[j - 1] {
      arr.swap(j, j - 1);
      j -= 1;
    }
  }
}

/// Wraps the Quick Sort algorithm for sorting an array in place.
///
/// # Arguments
///
/// * `arr` - A mutable slice of elements that implement the `PartialOrd` and `Copy` traits.
///
/// # Example
///
/// ```
/// let mut arr = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
/// quick_sort(&mut arr);
/// assert_eq!(arr, [1, 1, 2, 3, 3, 4, 5, 5, 5, 6, 9]);
/// ```
pub fn quick_sort<T: PartialOrd + Copy>(arr: &mut [T]) {
  if arr.is_empty() {
    return;
  }
  recursive_quick_sort(arr, 0, arr.len() - 1);
}

/// Recursively sorts an array in place using the Quick Sort algorithm.
///
/// # Arguments
///
/// * `arr` - A mutable slice of elements that implement the `PartialOrd` and `Copy` traits.
/// * `lower_bound` - The starting index of the slice to be sorted.
/// * `upper_bound` - The ending index of the slice to be sorted.
///
/// This function is called by `quick_sort` and should not be used directly.
///
/// # Example
///
/// ```
/// let mut arr = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
/// recursive_quick_sort(&mut arr, 0, arr.len() - 1);
/// assert_eq!(arr, [1, 1, 2, 3, 3, 4, 5, 5, 5, 6, 9]);
/// ```
fn recursive_quick_sort<T: PartialOrd + Copy>(
  arr: &mut [T],
  lower_bound: usize,
  upper_bound: usize,
) {
  if lower_bound >= upper_bound {
    return;
  }
  let pivot_index = partition(arr, lower_bound, upper_bound);
  if pivot_index > 0 {
    recursive_quick_sort(arr, lower_bound, pivot_index - 1);
  }
  recursive_quick_sort(arr, pivot_index + 1, upper_bound);
}

/// Partitions the array around a pivot element for the Quick Sort algorithm.
///
/// This function selects the last element as the pivot and rearranges the array
/// such that all elements less than the pivot are on the left, and all elements
/// greater than or equal to the pivot are on the right. It then returns the index
/// of the pivot element after partitioning.
///
/// # Arguments
///
/// * `arr` - A mutable slice of elements that implement the `PartialOrd` and `Copy` traits.
/// * `lower_bound` - The starting index of the slice to be partitioned.
/// * `upper_bound` - The ending index of the slice to be partitioned.
///
/// # Returns
///
/// The index of the pivot element after partitioning.
///
/// # Example
///
/// ```
/// let mut arr = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
/// let pivot_index = partition(&mut arr, 0, arr.len() - 1);
/// assert_eq!(pivot_index, 4); // Example index, actual value may vary
/// ```
fn partition<T: PartialOrd + Copy>(arr: &mut [T], lower_bound: usize, upper_bound: usize) -> usize {
  let pivot = arr[upper_bound];
  let mut left_item = lower_bound as isize - 1;
  for right_item in lower_bound..upper_bound {
    if arr[right_item] < pivot {
      left_item += 1;
      arr.swap(left_item as usize, right_item);
    }
  }
  let new_pivot = (left_item + 1) as usize;
  arr.swap(new_pivot, upper_bound);
  new_pivot
}
// TODO: Merge sort

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_selection_sort() {
    let mut arr = [5, 3, 2, 4, 1];
    selection_sort(&mut arr);
    assert_eq!(arr, [1, 2, 3, 4, 5]);
  }

  #[test]
  fn test_bubble_sort() {
    let mut arr = [5, 3, 2, 4, 1];
    bubble_sort(&mut arr);
    assert_eq!(arr, [1, 2, 3, 4, 5]);
  }

  #[test]
  fn test_insertion_sort() {
    let mut arr = [5, 3, 2, 4, 1];
    insertion_sort(&mut arr);
    assert_eq!(arr, [1, 2, 3, 4, 5]);
  }

  #[test]
  fn test_quick_sort() {
    let mut arr = [5, 3, 2, 4, 1];
    quick_sort(&mut arr);
    assert_eq!(arr, [1, 2, 3, 4, 5]);
  }

  // #[test]
  // fn test_merge_sort() {
  //     let mut arr = [5, 3, 2, 4, 1];
  //     merge_sort(&mut arr);
  //     assert_eq!(arr, [1, 2, 3, 4, 5]);
  // }
}
