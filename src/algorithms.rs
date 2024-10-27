/**
Sorts a slice in place using the selection sort algorithm.

Selection sort is an in-place comparison sorting algorithm. It divides the input list into two parts: the sublist of items already sorted, which is built up from left to right at the front (left) of the list, and the sublist of items remaining to be sorted that occupy the rest of the list. Initially, the sorted sublist is empty, and the unsorted sublist is the entire input list. The algorithm proceeds by finding the smallest (or largest, depending on sorting order) element from the unsorted sublist, swapping it with the leftmost unsorted element (putting it in sorted order), and moving the sublist boundaries one element to the right.

# Arguments

* `arr` - A mutable slice of elements that implement the `PartialOrd` trait. This slice will be sorted in place.

# Example

```
let mut arr = vec![5, 3, 2, 4, 1];
selection_sort(&mut arr);
assert_eq!(arr, vec![1, 2, 3, 4, 5]);
```

# Characteristics

* Stable: No
* Adaptive: No
* In-place: Yes
*/
pub fn selection_sort<T: PartialOrd>(arr: &mut [T]) {
  todo!()
}

/**
Sorts an array in place using the bubble sort algorithm

Bubble sort is a simple sorting algorithm that repeatedly steps through the list, compares adjacent elements, and swaps them if they are in the wrong order. The pass through the list is repeated until the list is sorted. The algorithm gets its name because smaller elements "bubble" to the top of the list.

# Arguments

* `arr` - A mutable slice of elements that implement the `PartialOrd` trait

# Example

```
let mut arr = [5, 3, 8, 4, 2];
bubble_sort(&mut arr);
assert_eq!(arr, [2, 3, 4, 5, 8]);
```
# Characteristics

* Stable: Yes
* Adaptive: Yes, if implemented with a flag to stop early if no swaps are made in a pass.
* In-place: Yes
*/
pub fn bubble_sort<T: PartialOrd>(arr: &mut [T]) {
  todo!()
}

/**
Sorts a slice in place using the insertion sort algorithm.

It iterates over the slice and maintains a sorted sublist in the lower positions of the list. Each new item is then "inserted" back into the previous sublist such that the sorted sublist is one item larger.

# Arguments

* `arr` - A mutable slice of elements implementing `PartialOrd`.

# Example

```
let mut arr = vec![5, 3, 2, 4, 1];
insertion_sort(&mut arr);
assert_eq!(arr, vec![1, 2, 3, 4, 5]);
```
# Characteristics

* Stable: Yes
* Adaptive: Yes, if implemented with a flag to stop early if no swaps are made in a pass.
* In-place: Yes
*/
pub fn insertion_sort<T: PartialOrd>(arr: &mut [T]) {
  todo!()
}

/**
Wraps the Quick Sort algorithm for sorting an array in place.

It works by selecting a 'pivot' element from the array and partitioning the other elements into two sub-arrays, according to whether they are less than or greater than the pivot. The sub-arrays are then sorted recursively.

# Arguments

* `arr` - A mutable slice of elements that implement the `PartialOrd` and `Copy` traits.

# Example

```
let mut arr = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
quick_sort(&mut arr);
assert_eq!(arr, [1, 1, 2, 3, 3, 4, 5, 5, 5, 6, 9]);
```

# Characteristics

* Stable: No
* Adaptive: No
* In-place: Yes
*/
pub fn quick_sort<T: PartialOrd + Copy>(arr: &mut [T]) {
  todo!()
}

/**
Partitions the array around a pivot element for the Quick Sort algorithm.

This function selects the last element as the pivot and rearranges the array
such that all elements less than the pivot are on the left, and all elements
greater than or equal to the pivot are on the right. It then returns the index
of the pivot element after partitioning.

# Arguments

* `arr` - A mutable slice of elements that implement the `PartialOrd` and `Copy` traits.
* `lower_bound` - The starting index of the slice to be partitioned.
* `upper_bound` - The ending index of the slice to be partitioned.

# Returns

The index of the pivot element after partitioning.

# Example

```
let mut arr = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
let pivot_index = partition(&mut arr, 0, arr.len() - 1);
assert_eq!(pivot_index, 4); // Example index, actual value may vary
```
*/
fn partition<T: PartialOrd + Copy>(arr: &mut [T], lower_bound: usize, upper_bound: usize) -> usize {
  todo!()
}

/**
Sorts an array using a iterative version of the Merge Sort algorithm.

It works by recursively dividing the array into two halves, sorting each half, and then merging the sorted halves back together.

# Arguments

* `arr` - A mutable slice of elements that implement `PartialOrd` and `Copy`.

# Example

```rust
let mut arr = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
merge_sort(&mut arr);
assert_eq!(arr, [1, 1, 2, 3, 3, 4, 5, 5, 5, 6, 9]);
```

# Characteristics

* Stable: Yes
* Adaptive: No
* In-place: No
*/
pub fn merge_sort<T: PartialOrd + Copy>(arr: &mut [T]) {
  todo!()
}

/**
Merges two sorted slices into a single sorted slice.

# Arguments

* `arr` - A mutable slice where the merged result will be stored.
* `left_arr` - A slice containing the left half of the sorted elements.
* `right_arr` - A slice containing the right half of the sorted elements.

# Example

```rust
let left_arr = [1, 3, 5];
let right_arr = [2, 4, 6];
let mut arr = [0; 6];
merge(&mut arr, &left_arr, &right_arr);
assert_eq!(arr, [1, 2, 3, 4, 5, 6]);

```
# Characteristics

* Stable: Yes
* Adaptive: No
* In-place: No
*/
fn merge<T: PartialOrd + Copy>(arr: &mut [T], left_arr: &[T], right_arr: &[T]) {
  todo!()
}

#[cfg(test)]
mod tests {
  use super::*;

  fn reverse_list_test(func: fn(&mut [i32])) {
    let mut arr = [5, 3, 2, 4, 1];
    func(&mut arr);
    assert_eq!(arr, [1, 2, 3, 4, 5], "reverse_list_test failed");
  }

  fn duplicates_list_test(func: fn(&mut [i32])) {
    let mut arr = [4, 2, 3, 2, 1, 4];
    func(&mut arr);
    assert_eq!(arr, [1, 2, 2, 3, 4, 4], "duplicates_list_test failed");
  }

  fn already_sorted_list_test(func: fn(&mut [i32])) {
    let mut arr = [1, 2, 3, 4, 5];
    func(&mut arr);
    assert_eq!(arr, [1, 2, 3, 4, 5], "already_sorted_list_test failed");
  }

  fn singleton_list_test(func: fn(&mut [i32])) {
    let mut arr = [42];
    func(&mut arr);
    assert_eq!(arr, [42], "singleton_list_test faild");
  }

  fn empty_list_test(func: fn(&mut [i32])) {
    let mut arr: [i32; 0] = [];
    func(&mut arr);
    assert_eq!(arr, [], "empty_list_test failed");
  }

  #[test]
  fn test_selection_sort() {
    reverse_list_test(selection_sort);
    duplicates_list_test(selection_sort);
    already_sorted_list_test(selection_sort);
    singleton_list_test(selection_sort);
    empty_list_test(selection_sort);
  }

  #[test]
  fn test_bubble_sort() {
    reverse_list_test(bubble_sort);
    duplicates_list_test(bubble_sort);
    already_sorted_list_test(bubble_sort);
    singleton_list_test(bubble_sort);
    empty_list_test(bubble_sort);
  }

  #[test]
  fn test_insertion_sort() {
    reverse_list_test(insertion_sort);
    duplicates_list_test(insertion_sort);
    already_sorted_list_test(insertion_sort);
    singleton_list_test(insertion_sort);
    empty_list_test(insertion_sort);
  }

  #[test]
  fn test_quick_sort() {
    reverse_list_test(quick_sort);
    duplicates_list_test(quick_sort);
    already_sorted_list_test(quick_sort);
    singleton_list_test(quick_sort);
    empty_list_test(quick_sort);
  }

  #[test]
  fn test_merge_sort() {
    reverse_list_test(merge_sort);
    duplicates_list_test(merge_sort);
    already_sorted_list_test(merge_sort);
    singleton_list_test(merge_sort);
    empty_list_test(merge_sort);
  }
}
