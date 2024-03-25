#![allow(dead_code)]
pub type UnsignedCounter = usize;
#[derive(PartialEq, Debug)]
struct UnsignedCounterStruc {
    cntr: UnsignedCounter,
}

impl UnsignedCounterStruc {
    pub fn default_unsigned_counter() -> Self {
        Self { cntr: (0) }
    }

    pub fn next_unsigned(counter: &UnsignedCounterStruc) -> UnsignedCounterStruc {
        UnsignedCounterStruc {
            cntr: counter.cntr + 1,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(
            UnsignedCounterStruc::default_unsigned_counter(),
            UnsignedCounterStruc { cntr: 0 }
        );
        assert_eq!(
            UnsignedCounterStruc::next_unsigned(&UnsignedCounterStruc { cntr: 1 }),
            UnsignedCounterStruc { cntr: 2 }
        );
    }
}
