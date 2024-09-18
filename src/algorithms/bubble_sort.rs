fn bubble_sort(arr: &mut [i32]) {
    let len = arr.len();
    let size = len - 1;

    for i in 0..size {
        for j in 0..(size - i) {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut vec = vec![67, 22, 3, 2, 1];
        bubble_sort(&mut vec);
        assert_eq!(vec, vec![1, 2, 3, 22, 67]);
    }
}
