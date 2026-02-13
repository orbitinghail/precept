use rand::Rng;

/// Returns a random number generator that uses the dispatcher for randomness.
///
/// When precept is enabled, this returns an RNG that sources randomness from
/// the dispatcher, enabling deterministic replay. When disabled, it returns
/// the standard random generator.
#[cfg(not(feature = "enabled"))]
pub fn rng() -> impl Rng {
    rand::rng()
}

/// Returns a random number generator that uses the dispatcher for randomness.
///
/// When precept is enabled, this returns an RNG that sources randomness from
/// the dispatcher, enabling deterministic replay. When disabled, it returns
/// the standard random generator.
#[cfg(feature = "enabled")]
pub fn rng() -> impl Rng {
    DispatchRng
}

#[cfg(feature = "enabled")]
struct DispatchRng;

#[cfg(feature = "enabled")]
impl rand::TryRng for DispatchRng {
    type Error = core::convert::Infallible;

    fn try_next_u32(&mut self) -> Result<u32, Self::Error> {
        Ok(self.try_next_u64()? as u32)
    }

    fn try_next_u64(&mut self) -> Result<u64, Self::Error> {
        Ok(crate::dispatch::get_random())
    }

    fn try_fill_bytes(&mut self, dst: &mut [u8]) -> Result<(), Self::Error> {
        rand_core::utils::fill_bytes_via_next_word(dst, || self.try_next_u64())
    }
}

#[cfg(test)]
mod tests {
    use super::rng;
    use rand::RngExt;

    #[test]
    fn rng_generates_primitive_values() {
        let mut rng = rng();

        let _: u8 = rng.random();
        let _: u64 = rng.random();
        let _: bool = rng.random();
    }

    #[test]
    fn rng_can_fill_byte_buffer() {
        let mut rng = rng();
        let mut bytes = [0_u8; 32];

        rng.fill(&mut bytes);
    }
}
