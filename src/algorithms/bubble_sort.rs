fn bubble_sort(arr: &mut [i32]) {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut vec = vec![67, 22, 3];
        bubble_sort(&mut vec)
    }
}
