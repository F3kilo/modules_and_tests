#![allow(dead_code)]
pub type UnsignedCounter = usize;

pub fn default_unsigned_counter() -> UnsignedCounter {
    0
}

pub fn next_unsigned(counter: UnsignedCounter) -> UnsignedCounter {
    counter + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(default_unsigned_counter(), 0);
        assert_eq!(next_unsigned(1), 2);
    }
}
