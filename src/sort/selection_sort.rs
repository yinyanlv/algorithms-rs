use super::super::util::swap;

pub fn selection_sort(arr: &mut Vec<i32>) -> &mut Vec<i32> {
    let len = arr.len();
    let mut min_index;

    for i in 0..(len - 1) {
        min_index = i;
        for j in (min_index + 1)..len {
            if arr[j] < arr[min_index] {
                min_index = j;
            }
        }

        swap(arr, i, min_index);
    }

    return arr;
}

#[test]
fn test_selection_sort() {
    let mut a = vec![3, 2, 4, 3, 1];

    let b = selection_sort(&mut a);


    assert_eq!(b[0], 1);
}