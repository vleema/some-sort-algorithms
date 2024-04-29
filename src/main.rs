use std::time::Instant;

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
fn selection_sort<T: PartialOrd>(arr: &mut [T]) {
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

fn bubble_sort<T: PartialOrd>(arr: &mut [T]) {
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
fn insertion_sort<T: PartialOrd>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j] < arr[j - 1] {
            arr.swap(j, j - 1);
            j -= 1;
        }
    }
}

// TODO: Quick sort
// TODO: Merge sort

fn header(test: &str) {
    let padding = "=".repeat(12);
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

    #[test]
    fn test_merge_sort() {
        let mut arr = [5, 3, 2, 4, 1];
        merge_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }
}

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
    // {
    //     let mut arr = [98, 33, 24, 69, 11, 2, 42, 7];
    //     header("quick sort");
    //     body(quick_sort, &mut arr);
    // }
    // {
    //     let mut arr = [35, 12, 2, 90, 42, 78, 9, 10];
    //     header("merge sort");
    //     body(merge_sort, &mut arr);
    // }
}
