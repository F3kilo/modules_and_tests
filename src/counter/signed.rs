#[derive(PartialEq, Debug, Default)]
pub struct Signed(isize);

impl Signed {
    pub fn new() -> Signed {
        Signed(0)
    }

    pub fn next(self) -> Signed {
        Signed(self.0 + 1)
    }

    pub fn prev(self) -> Signed {
        Signed(self.0 - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_works() {
        assert_eq!(Signed(0), Signed::new());
    }

    #[test]
    fn next_works() {
        assert_eq!(Signed(1), Signed::new().next());
        assert_eq!(Signed(0), Signed(-1).next());
    }

    #[test]
    fn prev_works() {
        assert_eq!(Signed(0), Signed(1).prev());
        assert_eq!(Signed(-1), Signed::new().prev());
    }
}
