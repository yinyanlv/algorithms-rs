use super::super::util::swap;

pub fn bubble_sort(arr: &mut Vec<i32>) -> &mut Vec<i32> {
    let len = arr.len();

    for i in 0..(len - 1) {

        for j in 0..(len - i - 1) {

            if arr[j] > arr[j + 1] {
                swap(arr, j, j + 1);
            }
        }
    }

    arr
}

#[test]
fn test_bubble_sort() {
    let mut a = vec![3, 2, 4, 3, 1];

    let b = bubble_sort(&mut a);

    assert_eq!(b[0], 1);
}