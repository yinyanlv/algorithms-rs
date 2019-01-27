pub fn shell_sort<T: PartialOrd + Copy>(arr: &mut [T]) -> &mut [T] {
    let len = arr.len();
    let mut gap = 1;

    if len <= 1 {
        return arr;
    }

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


#[cfg(test)]
mod tests {
    use super::*;

    /// 测试空数组
    #[test]
    fn test_empty_array() {
        let mut a: Vec<i32> = vec![];

        let b = shell_sort(&mut a);

        assert_eq!(b, []);
    }

    /// 测试数组中包含奇数个成员
    #[test]
    fn test_odd_array() {
        let mut a = [3, 2, 4, -3, 1];

        let b = shell_sort(&mut a);

        assert_eq!(b, [-3, 1, 2, 3, 4]);
    }

    /// 测试数组中包含偶数个成员
    #[test]
    fn test_even_array() {
        let mut a = [-3, -4, 2, 1];

        let b = shell_sort(&mut a);

        assert_eq!(b, [-4, -3, 1, 2]);
    }

    #[bench]
    fn bench_shell_sort(b: &mut test::Bencher) {
        b.iter(|| {
            let mut arr: Vec<u32> = (0..1000).rev().collect();

            shell_sort(&mut arr);
        });
    }
}