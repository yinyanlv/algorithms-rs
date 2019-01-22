pub fn swap(arr: &mut Vec<i32>, i: usize, j: usize) -> &mut Vec<i32> {
    let temp;

    temp = arr[i];
    arr[i] = arr[j];
    arr[j] = temp;

    return arr;
}

pub fn clone_slice_into_vec<T: Copy>(arr: &[T]) -> Vec<T> {
    let mut result = vec![];
    let len = arr.len();

    for i in 0..len {
        result.push(arr[i]);
    }

    return result;
}