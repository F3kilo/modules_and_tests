#[derive(PartialEq, Debug, Default)]
pub struct Pair(i32, i32);

impl Pair {
    pub fn new() -> Pair {
        Pair(0, 0)
    }

    pub fn vector_sum(self, other: Pair) -> Pair {
        Pair(self.0 + other.0, self.1 + other.1)
    }

    pub fn scalar_sum(self, other: Pair) -> i32 {
        self.0 + self.1 + other.0 + other.1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_works() {
        assert_eq!(Pair(0, 0), Pair::new());
    }

    #[test]
    fn vector_sum_works() {
        assert_eq!(Pair(11, 50), Pair(10, 39).vector_sum(Pair(1, 11)));
    }

    #[test]
    fn scalar_sum_works() {
        assert_eq!(10, Pair(1, 2).scalar_sum(Pair(3, 4)));
    }
}
