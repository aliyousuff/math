pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn subtract(left: isize, right: isize ) -> isize {
    left - right 
}

pub fn multiplication(left: isize, right: isize) -> isize{
    left * right
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
