pub fn bubble_sort<T: PartialOrd + Copy>(arr: &mut [T]) -> &mut [T] {
    let len = arr.len();

    if len <= 1 {
        return arr;
    }

    for i in 0..(len - 1) {
        for j in 0..(len - i - 1) {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1)
            }
        }
    }

    arr
}


#[cfg(test)]
mod tests {
    use super::*;

    /// 测试空数组
    #[test]
    fn test_empty_array() {
        let mut a: Vec<i32> = vec![];

        let b = bubble_sort(&mut a);

        assert_eq!(b, []);
    }

    /// 测试数组中包含奇数个成员
    #[test]
    fn test_odd_array() {
        let mut a = [3, 2, 4, -3, 1];

        let b = bubble_sort(&mut a);

        assert_eq!(b, [-3, 1, 2, 3, 4]);
    }

    /// 测试数组中包含偶数个成员
    #[test]
    fn test_even_array() {
        let mut a = [-3, -4, 2, 1];

        let b = bubble_sort(&mut a);

        assert_eq!(b, [-4, -3, 1, 2]);
    }

    #[bench]
    fn bench_bubble_sort(b: &mut test::Bencher) {
        b.iter(|| {
            let mut arr: Vec<u32> = (0..1000).rev().collect();

            bubble_sort(&mut arr);
        });
    }
}
