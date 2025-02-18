pub fn prob_1(n: i32) -> i32 {
    let mut sum = 0;
    for i in 0..n {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let sum = prob_1(10);
        assert_eq!(sum, 23);
    }
}
