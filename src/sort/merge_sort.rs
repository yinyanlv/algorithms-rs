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

#[test]
fn test_merge_sort() {
    let mut a = [3, 2, 4, -3, 1];

    let b = merge_sort(&mut a);

    assert_eq!(b, [-3, 1, 2, 3, 4]);
}