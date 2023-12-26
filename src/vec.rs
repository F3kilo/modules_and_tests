const LEN: usize = 3;

pub type Vec3 = [i32; LEN];

pub fn default() -> Vec3 {
    [0; 3]
}

pub fn vector_sum(a: Vec3, b: Vec3) -> Vec3 {
    let mut c = default();
    for i in 0..3 {
        c[i] = a[i] + b[i];
    }
    c
}

pub fn scalar_sum(a: Vec3, b: Vec3) -> i32 {
    let mut c = 0;
    for i in 0..LEN {
        c += a[i] + b[i];
    }
    c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_works() {
        assert_eq!([0, 0, 0] as Vec3, default());
    }

    #[test]
    fn vector_sum_works() {
        assert_eq!([2, 6, 10] as Vec3, vector_sum([1, 4, 8], [1, 2, 2]));
    }

    #[test]
    fn scalar_sum_works() {
        assert_eq!(42, scalar_sum([1, 5, 11], [6, 9, 10]))
    }
}
