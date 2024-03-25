#![allow(dead_code)]
pub type SignedCounter = isize;
#[derive(PartialEq, Debug)]
struct SignedCounterStruc {
    cntr: SignedCounter,
}
impl SignedCounterStruc {
    pub fn default_signed_counter() -> Self {
        Self { cntr: (0) }
    }

    pub fn next_signed(counter: &SignedCounterStruc) -> SignedCounterStruc {
        SignedCounterStruc {
            cntr: counter.cntr + 1,
        }
    }

    pub fn prev_signed(counter: &SignedCounterStruc) -> SignedCounterStruc {
        SignedCounterStruc {
            cntr: counter.cntr - 1,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(
            SignedCounterStruc::default_signed_counter(),
            SignedCounterStruc { cntr: 0 }
        );
        assert_eq!(
            SignedCounterStruc::next_signed(&SignedCounterStruc { cntr: 1 }),
            SignedCounterStruc { cntr: 2 }
        );
        assert_eq!(
            SignedCounterStruc::prev_signed(&SignedCounterStruc { cntr: 0 }),
            SignedCounterStruc { cntr: -1 }
        );
    }
}
