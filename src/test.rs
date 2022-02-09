#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use crate::insertion_sort;

        let to_sort = vec![1000000, 24289382, -10000, 293829, 292, 292982, 292239, -392];

        assert_eq!(
            insertion_sort(to_sort),
            vec![-10000, -392, 292, 292239, 292982, 293829, 1000000, 24289382]
        );
    }
}
