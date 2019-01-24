pub fn selection_sort<T: PartialOrd + Copy>(arr: &mut [T]) -> &mut [T] {
    let len = arr.len();
    let mut min_index;

    for i in 0..(len - 1) {
        min_index = i;
        for j in (min_index + 1)..len {
            if arr[j] < arr[min_index] {
                min_index = j;
            }
        }

        arr.swap(i, min_index);
    }

    return arr;
}

#[test]
fn test_selection_sort() {
    let mut a = [3, 2, 4, -3, 1];

    let b = selection_sort(&mut a);

    assert_eq!(b, [-3, 1, 2, 3, 4]);
}