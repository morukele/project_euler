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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prob_5_test() {
        let res = prob_5(10);
        assert_eq!(res, 2520);
    }
}
