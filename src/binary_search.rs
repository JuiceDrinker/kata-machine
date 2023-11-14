pub fn search<T>(haystack: &[T], needle: T) -> bool
where
    T: PartialEq + PartialOrd + Sized + Copy,
{
    let mut lo = 0;
    let mut hi = haystack.len();

    while lo < hi {
        let mid = f32::floor((lo + (hi - lo) / 2) as f32) as usize;
        let value = haystack[mid];

        if value == needle {
            return true;
        } else if value > needle {
            hi = mid;
        } else {
            lo = mid + 1
        }
    }
    false
}

mod test {
    use crate::binary_search::search;

    #[test]
    pub fn it_finds() {
        let haystack = [1, 31, 43, 68, 92, 100];

        assert!(search(&haystack, 31));
    }

    #[test]
    pub fn it_doesnt_find() {
        let haystack = [1, 31, 43, 68, 92, 100];

        assert!(!search(&haystack, 32));
    }
}
