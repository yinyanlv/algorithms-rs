pub fn quick_sort<T: PartialOrd + Copy>(arr: &mut [T], left: usize, right: usize) -> &mut [T] {
    let len = arr.len();

    if len <= 1 {
        return arr;
    }

    if left < right {
        if right - left == 1 {
            if arr[left] > arr[right] {
                arr.swap(left, right);
            }
        } else {
            let partition_index = partition(arr, left, right);

            if partition_index > 0 {
                quick_sort(arr, left, partition_index - 1);
            }
            quick_sort(arr, partition_index + 1, right);
        }
    }

    arr
}

fn partition<T: PartialOrd + Copy>(arr: &mut [T], left: usize, right: usize) -> usize {
    let pivot = left;
    let mut next = pivot + 1;

    while next <= right && arr[next] < arr[pivot] {
        next += 1;
    }

    for i in (next + 1)..=right {
        if arr[i] < arr[pivot] {
            arr.swap(next, i);
            next += 1;
        }
    }

    arr.swap(pivot, next - 1);

    return next - 1;
}

#[test]
fn test_quick_sort() {
    let mut a = [3, 2, 4, -3, 1];
    let len = a.len();

    let b = quick_sort(&mut a, 0, len - 1);

    assert_eq!(b, [-3, 1, 2, 3, 4]);
}