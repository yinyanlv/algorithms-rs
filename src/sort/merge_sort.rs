use super::super::util::{swap, clone_slice_into_vec};

pub fn merge_sort(mut arr: Vec<i32>) -> Vec<i32> {
    let len = arr.len();

    if len < 2 {

        return arr;
    } else if len == 2 {

        if arr[0] > arr[1] {
            swap(&mut arr, 0, 1);
        }
        return arr;
    }

    let mid_index = len / 2;

    let arr1 = clone_slice_into_vec(&arr[0..mid_index]);
    let arr2 = clone_slice_into_vec(&arr[mid_index..len]);

    return merge(merge_sort(arr1), merge_sort(arr2));
}

fn merge(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
    let mut arr: Vec<i32> = vec![];
    let len1 = arr1.len();
    let len2 = arr2.len();
    let len = len1 + len2;

    let mut i = 0;
    let mut j = 0;

    for _n in 0..len {

        if i >= len1 {
            arr.push(arr2[j]);
            j += 1;
        } else if j >= len2 {
            arr.push(arr1[i]);
            i += 1;
        } else if arr1[i] < arr2[j] {
            arr.push(arr1[i]);
            i += 1;
        } else {
            arr.push(arr2[j]);
            j += 1;
        }
    }

    return arr;
}

#[test]
fn test_merge_sort() {
    let a = vec![3, 2, 4, 3, 1];

    let b = merge_sort(a);

    assert_eq!(b[0], 1);
}