pub fn bubble_sort(arr: &mut Vec<i32>) -> &mut Vec<i32> {
    let len = arr.len();
    let mut temp;

    for i in 0..(len - 1) {

        for j in 0..(len - i - 1) {

            if arr[j] > arr[j + 1] {
                temp = arr[j];
                arr[j] = arr[j + 1];
                arr[j + 1] = temp;
            }
        }
    }

    arr
}

#[test]
fn test_bubble_sort() {
    let mut a = vec![3, 2, 4, 3, 1];

    let b = bubble_sort(&mut a);

    assert_eq!(b[0], 1);
}