use std::ops::{Index, IndexMut};

const LEN: usize = 3;

#[derive(PartialEq, Debug, Default)]
pub struct Vec3([i32; LEN]);

impl Index<usize> for Vec3 {
    type Output = i32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Vec3 {
    pub fn new() -> Vec3 {
        Vec3([0; 3])
    }

    pub fn vector_sum(self, other: Vec3) -> Vec3 {
        let mut c = Vec3::new();
        for i in 0..3 {
            c[i] = self[i] + other[i];
        }
        c
    }

    pub fn scalar_sum(self, other: Vec3) -> i32 {
        let mut c = 0;
        for i in 0..LEN {
            c += self[i] + other[i];
        }
        c
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_works() {
        assert_eq!(Vec3([0, 0, 0]), Vec3::new());
    }

    #[test]
    fn vector_sum_works() {
        assert_eq!(
            Vec3([2, 6, 10]),
            Vec3([1, 4, 8]).vector_sum(Vec3([1, 2, 2]))
        );
    }

    #[test]
    fn scalar_sum_works() {
        assert_eq!(42, Vec3([1, 5, 11]).scalar_sum(Vec3([6, 9, 10])))
    }
}
