pub fn shell_sort<T: PartialOrd + Copy>(arr: &mut [T]) -> &mut [T] {
    let len = arr.len();
    let mut gap = 1;

    while gap < (len / 3) {
        gap = gap * 3 + 1;
    }

    while gap > 0 {
        for i in (gap - 1)..(len - gap) {
            let mut j = i + gap;

            while (j >= gap) && (arr[j] < arr[j - gap]) {
                arr.swap(j - gap, j);
                j -= gap;
            }
        }

        gap = gap / 3;
    }

    arr
}

#[test]
fn test_shell_sort() {
    let mut a = [3, 2, 4, -3, 1];

    let b = shell_sort(&mut a);

    assert_eq!(b, [-3, 1, 2, 3, 4]);
}
