#![allow(dead_code)]
#[derive(PartialEq, Debug)]
pub struct Pair {
    x: i32,
    y: i32,
}

impl Pair {
    pub fn default_pair() -> Self {
        Self { x: 0, y: 0 }
    }

    pub fn pair_vector_sum(a: &Pair, b: &Pair) -> Self {
        Self {
            x: (a.x + b.x),
            y: (a.y + b.y),
        }
    }

    pub fn pair_scalar_sum(a: &Pair, b: &Pair) -> i32 {
        a.x + a.y + b.x + b.y
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(Pair::default_pair(), Pair { x: 0, y: 0 });
        assert_eq!(
            Pair::pair_vector_sum(&Pair { x: 1, y: 1 }, &Pair { x: 2, y: 2 }),
            Pair { x: 3, y: 3 }
        );
        assert_eq!(
            Pair::pair_scalar_sum(&Pair { x: 1, y: 1 }, &Pair { x: 2, y: 2 }),
            6
        );
    }
}
