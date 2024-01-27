#[derive(PartialEq, Debug, Default)]
pub struct Unsigned(usize);

impl Unsigned {
    pub fn new() -> Unsigned {
        Unsigned(0)
    }

    pub fn next(self) -> Unsigned {
        Unsigned(self.0 + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_works() {
        assert_eq!(Unsigned(0), Unsigned::new());
    }

    #[test]
    fn next_works() {
        assert_eq!(Unsigned(1), Unsigned::new().next());
        assert_eq!(Unsigned(2), Unsigned(1).next());
    }
}
