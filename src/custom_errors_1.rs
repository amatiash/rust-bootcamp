#![allow(dead_code, unused_variables)]

pub fn code() {
    // See basic_error_handling.rs for the code
}

pub fn creating_custom_errors() {
    // // Complete the `factorial` function.
    // #[derive(Debug)]
    // enum Error {
    //     Overflow,
    //     InvalidInput,
    // }
    // 
    // fn factorial(num: i32) -> Result<i32, Error> {
    //     /*
    //        if num < 0 return InvalidInput error
    //        if num > 12 return Overflow error
    //        if num < 2 return num
    //        else return num * result of factorial num - 1
    //     */
    // }
    // 
    // fn main() {
    //     assert!(matches!(factorial(-12), Err(Error::InvalidInput)));
    //     assert!(matches!(factorial(20), Err(Error::Overflow)));
    //     assert!(matches!(factorial(5), Ok(120)));
    // }
    
    #[derive(Debug)]
    enum Error {
        Overflow,
        InvalidInput,
    }

    fn factorial(num: i32) -> Result<i32, Error> {
        /*
           if num < 0 return InvalidInput error
           if num > 12 return Overflow error
           if num < 2 return num
           else return num * result of factorial num - 1
        */
        if num < 0 {
            return Err(Error::InvalidInput);
        } else if num > 12 {
            return Err(Error::Overflow);
        } else if num < 2 {
            return Ok(num);
        }
        let result = factorial(num - 1)?;
        Ok(num * result)
    }

    assert!(matches!(factorial(-12), Err(Error::InvalidInput)));
    assert!(matches!(factorial(20), Err(Error::Overflow)));
    assert!(matches!(factorial(5), Ok(120)));
}
