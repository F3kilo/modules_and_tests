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
    }

    pub mod unsigned {
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
}
