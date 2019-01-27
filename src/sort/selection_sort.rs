pub fn selection_sort<T: PartialOrd + Copy>(arr: &mut [T]) -> &mut [T] {
    let len = arr.len();
    let mut min_index;

    if len <= 1 {
        return arr;
    }

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


#[cfg(test)]
mod tests {
    use super::*;

    /// 测试空数组
    #[test]
    fn test_empty_array() {
        let mut a: Vec<i32> = vec![];

        let b = selection_sort(&mut a);

        assert_eq!(b, []);
    }

    /// 测试数组中包含奇数个成员
    #[test]
    fn test_odd_array() {
        let mut a = [3, 2, 4, -3, 1];

        let b = selection_sort(&mut a);

        assert_eq!(b, [-3, 1, 2, 3, 4]);
    }

    /// 测试数组中包含偶数个成员
    #[test]
    fn test_even_array() {
        let mut a = [-3, -4, 2, 1];

        let b = selection_sort(&mut a);

        assert_eq!(b, [-4, -3, 1, 2]);
    }

    #[bench]
    fn bench_selection_sort(b: &mut test::Bencher) {
        b.iter(|| {
            let mut arr: Vec<u32> = (0..1000).rev().collect();

            selection_sort(&mut arr);
        });
    }
}
