pub mod counter {
    pub mod signed {
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
    }

    pub mod unsigned {
        pub type Unsigned = usize;
        pub fn default() -> Unsigned {
            0
        }
        pub fn next(counter: Unsigned) -> Unsigned {
            counter + 1
        }
    }
}

pub mod pair {
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
}

pub mod vec {
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
}
