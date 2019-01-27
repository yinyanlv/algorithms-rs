pub fn merge_sort<T: PartialOrd + Copy>(arr: &mut [T]) -> &mut [T] {
    let len = arr.len();

    if len < 2 {
        return arr;
    } else if len == 2 {
        if arr[0] > arr[1] {
            arr.swap(0, 1);
        }
        return arr;
    }

    let mid_index = len / 2;

    merge_sort(&mut arr[0..mid_index]);
    merge_sort(&mut arr[mid_index..len]);

    return merge(arr);
}

fn merge<T: PartialOrd + Copy>(arr: &mut [T]) -> &mut [T] {
    let len = arr.len();
    let mid_index = len / 2;
    let len1 = mid_index;
    let len2 = len - len1;

    let mut arr1 = Vec::<T>::with_capacity(len1);
    let mut arr2 = Vec::<T>::with_capacity(len2);

    arr1.extend_from_slice(&arr[0..mid_index]);
    arr2.extend_from_slice(&arr[mid_index..len]);

    let mut i = 0;
    let mut j = 0;

    for n in 0..len {
        if i >= len1 {
            arr[n] = arr2[j];
            j += 1;
        } else if j >= len2 {
            arr[n] = arr1[i];
            i += 1;
        } else if arr1[i] < arr2[j] {
            arr[n] = arr1[i];
            i += 1;
        } else {
            arr[n] = arr2[j];
            j += 1;
        }
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

        let b = merge_sort(&mut a);

        assert_eq!(b, []);
    }

    /// 测试数组中包含奇数个成员
    #[test]
    fn test_odd_array() {
        let mut a = [3, 2, 4, -3, 1];

        let b = merge_sort(&mut a);

        assert_eq!(b, [-3, 1, 2, 3, 4]);
    }

    /// 测试数组中包含偶数个成员
    #[test]
    fn test_even_array() {
        let mut a = [-3, -4, 2, 1];

        let b = merge_sort(&mut a);

        assert_eq!(b, [-4, -3, 1, 2]);
    }

    #[bench]
    fn bench_merge_sort(b: &mut test::Bencher) {
        b.iter(|| {
            let mut arr: Vec<u32> = (0..1000).rev().collect();

            merge_sort(&mut arr);
        });
    }
}