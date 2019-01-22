use super::super::util::swap;

pub fn quick_sort(arr: &mut Vec<i32>, left: usize, right: usize) -> &mut Vec<i32> {
    let len = arr.len();

    if len == 0 {
        return arr;
    }

    let partition_index = partition(arr, left, right);

    if left < partition_index - 1 {
        quick_sort(arr, left, partition_index - 1);
    }

    if right > partition_index + 1 {
        quick_sort(arr, partition_index + 1, right);
    }

    arr
}

fn partition(arr: &mut Vec<i32>, left: usize, right: usize) -> usize {
    let pivot = left;
    let mut next = pivot + 1;

    for i in (next + 1)..(right + 1) {
        if arr[i] < arr[pivot] {
            swap(arr, next, i);
            next += 1;
        }
    }

    swap(arr, pivot, next - 1);

    return next - 1;
}

#[test]
fn test_quick_sort() {
    let mut a = vec![3, 2, 4, 3, 1];
    let len = a.len();

    let b = quick_sort(&mut a, 0, len - 1);

    assert_eq!(b[0], 1);
}