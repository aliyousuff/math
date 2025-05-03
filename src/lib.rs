// this function adds two numbers
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn subtract(left: isize, right: isize ) -> isize {
    left - right 
}

pub fn multiplication(left: isize, right: isize) -> isize{
    left * right
}

pub fn divide(left: isize, right: isize ) -> Result<isize, String>{
    if right == 0 {
        Err("Division by zero".to_string())
    } else {
        Ok(left / right)
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_works2() {
        let result = multiplication(2, 3);
        assert_eq!(result, 6)
    }

    #[test]
    fn it_works3() {
        let result = divide(10, 2);
        assert_eq!(result.unwrap(), 5);
    }

    #[test]
    fn it_works4() {
        let result = divide(9, 0);
        assert_eq!(result, Err("Division by zero".to_string()) )
    }
}
