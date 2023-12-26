pub type Unsigned = usize;
pub fn default() -> Unsigned {
    0
}
pub fn next(counter: Unsigned) -> Unsigned {
    counter + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_works() {
        assert_eq!(0 as Unsigned, default());
    }

    #[test]
    fn next_works() {
        assert_eq!(1 as Unsigned, next(0));
        assert_eq!(2 as Unsigned, next(1));
    }
}
