#[no_mangle]
pub fn xadd(left: u64, right: u64) -> u64 {
    left + right
}

#[no_mangle]
pub extern "C" fn yadd(a: u64, b: u64) -> u64 {
    a + b
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = xadd(2, 2);
        assert_eq!(result, 4);
        let r2 = yadd(2, 3);
        assert_eq!(r2, 5);
    }
}
