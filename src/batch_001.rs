pub fn prob_1(n: i32) -> i32 {
    let mut sum = 0;
    for i in 0..n {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }
    }

    sum
}

// A function that returns the sum of the even numbers in the n Fibonacci numbers
pub fn prob_2(n: i32) -> i32 {
    let mut total = 0;
    let (mut a, mut b) = (1, 1);

    while a <= n {
        // Check if the number is even
        if a % 2 == 0 {
            total += a;
        }
        (a, b) = (b, a + b); // update the Fibonacci sequence
    }

    total
}

// A function to return the largest prime factor of a number n
pub fn prob_3(mut n: i64) -> i64 {
    let mut divisor = 2;

    while n > 1 {
        if n % divisor == 0 {
            n /= divisor
        } else {
            divisor += 1;
        }
    }

    divisor
}

// A function that returns the largest palindromic number made from the product of two 3-digit numbers
pub fn prob_4() -> i64 {
    // THIS IS A GREEDY ALGORITHM
    let mut res = 0;
    for i in (100..=999).rev() {
        for j in (100..=i).rev() {
            let product = i * j;
            let string = product.to_string();
            let rstring: String = string.chars().rev().collect();
            if string == rstring && product > res {
                res = product
            }
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prob_1_test() {
        let sum = prob_1(10);
        assert_eq!(sum, 23);
    }

    #[test]
    fn prob_2_test() {
        let res = prob_2(89);
        assert_eq!(res, 44);
    }

    #[test]
    fn prob_3_test() {
        let res = prob_3(13195);
        assert_eq!(res, 29);
    }

    #[test]
    fn prob_4_test() {
        let res = prob_4();
        assert_eq!(res, 906609);
    }
}
