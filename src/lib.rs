pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn dub(i: usize) -> usize {
    2 * i
}

pub fn subtract(left: i32, right: i32) -> i32 {
    left - right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
