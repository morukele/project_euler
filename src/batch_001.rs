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

// A function that gives the smallest positive number that is evenly divisible by all  of the numbers from 1 to 20
pub fn prob_5(n: i64) -> i64 {
    // let mut solution = 0;
    // let mut solved = false;
    // let mut potential_solution = n; // using this because in theory the largest number must be able to divide the solution

    // while solved == false {
    //     for i in 1..=n {
    //         if potential_solution % i != 0 {
    //             break;
    //         } else {
    //             if i == n {
    //                 solution = potential_solution;
    //                 solved = true;
    //             }
    //         }
    //     }

    //     potential_solution += 1;
    // }

    // solution

    // - A more optimal solution proposed by Claude Sonnet 3.5
    let mut res = 1;
    for i in 1..=n {
        res = lcm(res, i);
    }

    res
}

// Calculate the greatest common divisor using Euclidean algorithm.
fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b > 0 {
        (a, b) = (b, a % b)
    }

    a
}

// Calculate the Least Common Multiple using GCD
fn lcm(a: i64, b: i64) -> i64 {
    (a * b) / gcd(a, b)
}

// A function to compute the difference between the sum of the squares and the squre of the sum of first n numbers
pub fn prob_6(n: i64) -> i64 {
    let mut sum_of_square = 0;
    let mut sum = 0;

    for i in 1..=n {
        // sume of the square
        sum_of_square += i * i;

        // squre of the sum
        sum += i;
    }

    (sum * sum) - sum_of_square
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

    #[test]
    fn prob_5_test() {
        let res = prob_5(10);
        assert_eq!(res, 2520);
    }

    #[test]
    fn prob_6_test() {
        let res = prob_6(10);
        assert_eq!(res, 2640);
    }
}
