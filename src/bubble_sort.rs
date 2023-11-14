fn bubble_sort<T: PartialOrd + Sized + Copy>(arr: &mut [T]) -> &[T] {
    for i in 0..arr.len() {
        for j in i..arr.len() {
            if arr[i] > arr[j] {
                arr.swap(i, j);
            }
        }
    }

    arr
}

mod test {
    use super::bubble_sort;

    #[test]

    fn it_sorts() {
        let mut arr = [9, 3, 7, 4, 69, 420, 42];

        bubble_sort(&mut arr);

        assert_eq!(arr, [3, 4, 7, 9, 42, 69, 420]);
    }
}
