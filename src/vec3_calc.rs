#![allow(dead_code)]

pub const VEC3_LEN: usize = 3;

pub type Vec3 = [i32; VEC3_LEN];
#[derive(PartialEq, Debug)]
pub struct Vec3Struc {
    vec3: Vec3,
}

impl Vec3Struc {
    pub fn default_vec3() -> Self {
        Self { vec3: [0; 3] }
    }

    pub fn vec3_vector_sum(a: &Vec3Struc, b: &Vec3Struc) -> Vec3Struc {
        let mut c = Vec3Struc::default_vec3();
        for i in 0..3 {
            c.vec3[i] = a.vec3[i] + b.vec3[i];
        }
        c
    }

    pub fn vec3_scalar_sum(a: &Vec3Struc, b: &Vec3Struc) -> i32 {
        let mut c = 0;
        for i in 0..VEC3_LEN {
            c += a.vec3[i] + b.vec3[i];
        }
        c
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(Vec3Struc::default_vec3(), Vec3Struc { vec3: [0, 0, 0] });
        assert_eq!(
            Vec3Struc::vec3_vector_sum(
                &Vec3Struc { vec3: [1, 2, 3] },
                &Vec3Struc { vec3: [4, 5, 6] }
            ),
            Vec3Struc { vec3: [5, 7, 9] }
        );
        assert_eq!(
            Vec3Struc::vec3_scalar_sum(
                &Vec3Struc { vec3: [1, 2, 3] },
                &Vec3Struc { vec3: [4, 5, 6] }
            ),
            21
        );
    }
}
