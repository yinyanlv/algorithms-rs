pub fn merge_sort(mut arr: Vec<i32>) -> Vec<i32> {
    let len = arr.len();

    if len < 2 {
        return arr;
    } else if len == 2 {
        let temp;
        if arr[0] > arr[1] {
            temp = arr[0];
            arr[0] = arr[1];
            arr[1] = temp;
        }
        return arr;
    }

    let mid_index = len / 2;

    return merge(&arr[0..mid_index], &arr[mid_index..len]);
}

fn merge(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let mut arr: Vec<i32> = Vec::new();
    let len1 = arr1.len();
    let len2 = arr2.len();
    let len = len1 + len2;

    let mut i = 0;
    let mut j = 0;

    if len1 == 0 && len2 == 0 {
        return arr;
    }

    if len1 == 0 && len2 != 0{
        for n in 0..len {
            arr.push(arr2[n]);
        }

        return arr;
    }

    if len1 != 0 && len2 == 0{
        for n in 0..len {
            arr.push(arr1[n]);
        }

        return arr;
    }

    for _n in 0..len {
        if i >= len1 {
            arr.push(arr2[j]);
            j += 1;
        } else if j >= len2 {
            arr.push(arr1[i]);
            i += 1;
        } else if arr[i] < arr[j] {
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