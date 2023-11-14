pub fn binary_search_tree<T: PartialEq>(haystack: &[T], needle: T) -> bool {
    for item in haystack {
        if needle == *item {
            return true;
        }
    }
    false
}

mod test {
    use super::*;

    #[test]
    fn it_returns_false() {
        let haystack = [1, 4, 19, 3242, 12, 4];

        assert!(!binary_search_tree(&haystack, 13))
    }
    #[test]
    fn it_returns_true() {
        let haystack = [1, 4, 19, 3242, 12, 4];

        assert!(binary_search_tree(&haystack, 12))
    }
}
