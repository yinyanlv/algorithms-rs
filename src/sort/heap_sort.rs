pub fn heap_sort<T: PartialOrd + Copy>(arr: &mut [T]) -> &mut [T] {
    let len = arr.len();

    if len <= 1 {
        return arr;
    }

    let mut i = len / 2 - 1;

    while i >= (0 as usize) {
        shift_down(arr, i, len);

        if i == 0 {
            break;
        }

        i -= 1;
    }

    let mut j = len - 1;

    while j > (0 as usize) {
        arr.swap(0, j);
        shift_down(arr, 0, j);

        j -= 1;
    }

    arr
}

/// 大顶堆
fn shift_down<T: PartialOrd + Copy>(arr: &mut [T], mut index: usize, count: usize) {
    let mut next = 2 * index + 1;

    while next < count {
        if (next + 1) < count && arr[next + 1] > arr[next] {
            next += 1;
        }

        if arr[index] > arr[next] {
            break;
        }

        arr.swap(index, next);

        index = next;
        next = 2 * index + 1;
    }
}

#[test]
fn test_heap_sort() {
    let mut a = [3, 2, 4, -3, 1];

    let b = heap_sort(&mut a);

    assert_eq!(b, [-3, 1, 2, 3, 4]);
}