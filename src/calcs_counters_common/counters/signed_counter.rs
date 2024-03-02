#![allow(dead_code)]
pub type SignedCounter = isize;

pub fn default_signed_counter() -> SignedCounter {
    0
}

pub fn next_signed(counter: SignedCounter) -> SignedCounter {
    counter + 1
}

pub fn prev_signed(counter: SignedCounter) -> SignedCounter {
    counter - 1
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(default_signed_counter(), 0);
        assert_eq!(next_signed(1), 2);
        assert_eq!(prev_signed(0), -1);
    }
}
