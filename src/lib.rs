//! A crate that contains some basic sorting algorithms.

/// A function for sorting an array of integers with insertion sort.
pub fn insertion_sort<T>(mut to_sort: Vec<T>) -> Vec<T>
where 
    T: PartialOrd + Copy
{
    for i in 1..to_sort.len() {
        let mut j = i;
        while j > 0 && to_sort[j - 1] > to_sort[j] {
            to_sort.swap(j - 1, j);
            j -= 1;
        }
    }
    to_sort
}
