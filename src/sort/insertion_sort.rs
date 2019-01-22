use super::super::util::swap;

pub fn insertion_sort(arr: &mut Vec<i32>) -> &mut Vec<i32> {
    let len = arr.len();

    for i in 0..(len - 1) {

        let mut j = i + 1;

        while j >= 1 && (arr[j] < arr[j - 1]) {
            swap(arr, j - 1, j);

            j -= 1;
        }
    }

    arr
}

#[test]
fn test_insertion_sort() {
    let mut a = vec![3, 2, 4, 3, 1];

    let b = insertion_sort(&mut a);

    assert_eq!(b[0], 1);
}