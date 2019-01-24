pub fn swap<T: PartialOrd + Copy>(arr: &mut [T], i: usize, j: usize) -> &mut [T] {
    let temp;

    temp = arr[i];
    arr[i] = arr[j];
    arr[j] = temp;

    return arr;
}
