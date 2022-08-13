mod sortings;

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_selection_sort() {
        let mut v = vec![2, 3, 89, 1, 6];
        let expected = vec![1, 2, 3, 6, 89];
        sortings::quadratic::insertion_sort(&mut v);

        assert_eq!(v, expected);
    }
}
