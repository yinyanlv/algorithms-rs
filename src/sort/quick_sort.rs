pub fn quick_sort<T: PartialOrd + Copy>(arr: &mut [T]) -> &mut [T] {

    let len = arr.len();

    if len <= 1 {
        return arr;
    }

    return sort(arr, 0, len - 1);
}

fn sort<T: PartialOrd + Copy>(arr: &mut [T], left: usize, right: usize) -> &mut [T] {
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
                sort(arr, left, partition_index - 1);
            }
            sort(arr, partition_index + 1, right);
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

#[cfg(test)]
mod tests {
    use super::*;

    /// 测试空数组
    #[test]
    fn test_empty_array() {
        let mut a: Vec<i32> = vec![];

        let b = quick_sort(&mut a);

        assert_eq!(b, []);
    }

    /// 测试数组中包含奇数个成员
    #[test]
    fn test_odd_array() {
        let mut a = [3, 2, 4, -3, 1];

        let b = quick_sort(&mut a);

        assert_eq!(b, [-3, 1, 2, 3, 4]);
    }

    /// 测试数组中包含偶数个成员
    #[test]
    fn test_even_array() {
        let mut a = [-3, -4, 2, 1];

        let b = quick_sort(&mut a);

        assert_eq!(b, [-4, -3, 1, 2]);
    }

    #[bench]
    fn bench_quick_sort(b: &mut test::Bencher) {
        b.iter(|| {
            let mut arr: Vec<u32> = (0..1000).rev().collect();

            quick_sort(&mut arr);
        });
    }
}

