pub fn insertion_sort<T: PartialOrd + Copy>(arr: &mut [T]) -> &mut [T] {
    let len = arr.len();

    for i in 0..(len - 1) {
        let mut j = i + 1;

        while j >= 1 && (arr[j] < arr[j - 1]) {
            arr.swap(j - 1, j);

            j -= 1;
        }
    }

    arr
}

#[test]
fn test_insertion_sort() {
    let mut a = [3, 2, 4, -3, 1];

    let b = insertion_sort(&mut a);

    assert_eq!(b, [-3, 1, 2, 3, 4]);
}