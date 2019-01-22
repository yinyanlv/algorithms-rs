use super::super::util::swap;

pub fn shell_sort(arr: &mut Vec<i32>) -> &mut Vec<i32> {
    let len = arr.len();
    let mut gap = 1;

    while gap < (len / 3) {
        gap = gap * 3 + 1;
    }

    while gap > 0 {
        for i in (gap - 1)..(len - gap) {
            let mut j = i + gap;

            while (j >= gap) && (arr[j] < arr[j - gap]) {
                swap(arr, j - gap, j);
                j -= gap;
            }
        }

        gap = gap / 3;
    }

    arr
}

#[test]
fn test_shell_sort() {
    let mut a = vec![3, 2, 4, 3, 1];

    let b = shell_sort(&mut a);

    assert_eq!(b[0], 1);
}
