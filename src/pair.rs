pub type Pair = (i32, i32);

pub fn default() -> Pair {
    (0, 0)
}

pub fn vector_sum(a: Pair, b: Pair) -> Pair {
    (a.0 + b.0, a.1 + b.1)
}

pub fn scalar_sum(a: Pair, b: Pair) -> i32 {
    a.0 + a.1 + b.0 + b.1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_works() {
        assert_eq!((0, 0) as Pair, default());
    }

    #[test]
    fn vector_sum_works() {
        assert_eq!((11, 50) as Pair, vector_sum((10, 39), (1, 11)));
    }

    #[test]
    fn scalar_sum_works() {
        assert_eq!(10, scalar_sum((1, 2), (3, 4)));
    }
}
