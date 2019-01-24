pub fn bubble_sort<T: PartialOrd + Copy>(arr: &mut [T]) -> &mut [T] {
    let len = arr.len();

    for i in 0..(len - 1) {
        for j in 0..(len - i - 1) {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1)
            }
        }
    }

    arr
}

#[test]
fn test_bubble_sort() {
    let mut a = [3, 2, 4, -3, 1];

    let b = bubble_sort(&mut a);

    assert_eq!(b, [-3, 1, 2, 3, 4]);
}