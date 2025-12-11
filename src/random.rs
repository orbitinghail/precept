use rand::RngCore;

#[cfg(not(feature = "enabled"))]
pub fn rng() -> impl RngCore {
    rand::rng()
}

#[cfg(feature = "enabled")]
pub fn rng() -> impl RngCore {
    DispatchRng
}

#[cfg(feature = "enabled")]
struct DispatchRng;

#[cfg(feature = "enabled")]
impl RngCore for DispatchRng {
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    fn next_u64(&mut self) -> u64 {
        crate::dispatch::get_random()
    }

    fn fill_bytes(&mut self, dst: &mut [u8]) {
        rand_core::impls::fill_bytes_via_next(self, dst)
    }
}
