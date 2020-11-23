//! [Problem 1](https://projecteuler.net/problem=1) Multiples of 3 and 5
//! If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.

//! Find the sum of all the multiples of 3 or 5 below 1000.

fn multiples_of_3_and_5(bound: u32) -> u32 {
    (1..bound).filter(|&n| n % 3 == 0 || n % 5 == 0).sum()
}


#[cfg(test)]
mod tests {
    use super::multiples_of_3_and_5;

    #[test]
    fn test() {
        assert_eq!(multiples_of_3_and_5(10), 23);
        println!("{}", multiples_of_3_and_5(1000));
    }
}