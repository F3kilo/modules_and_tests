pub type Signed = isize;
pub fn default() -> Signed {
    0
}
pub fn next(counter: Signed) -> Signed {
    counter + 1
}
pub fn prev(counter: Signed) -> Signed {
    counter - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_works() {
        assert_eq!(0 as Signed, default());
    }

    #[test]
    fn next_works() {
        assert_eq!(1 as Signed, next(0));
        assert_eq!(0 as Signed, next(-1));
    }

    #[test]
    fn prev_works() {
        assert_eq!(0 as Signed, prev(1));
        assert_eq!(-1 as Signed, prev(0));
    }
}
