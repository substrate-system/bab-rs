use zeroize::Zeroize;

#[derive(Eq, PartialOrd, Ord, Clone, Debug)]
#[repr(transparent)]
pub struct BabDigest<const WIDTH: usize>(pub(crate) [u8; WIDTH]);

impl<const WIDTH: usize> From<[u8; WIDTH]> for BabDigest<WIDTH> {
    fn from(value: [u8; WIDTH]) -> Self {
        Self(value)
    }
}

impl<const WIDTH: usize> From<BabDigest<WIDTH>> for [u8; WIDTH] {
    fn from(value: BabDigest<WIDTH>) -> Self {
        value.0
    }
}

impl<const WIDTH: usize> PartialEq for BabDigest<WIDTH> {
    fn eq(&self, other: &Self) -> bool {
        constant_time_eq::constant_time_eq_n(&self.0, &other.0)
    }
}

impl<const WIDTH: usize> Default for BabDigest<WIDTH> {
    fn default() -> Self {
        [0; WIDTH].into()
    }
}

impl<const WIDTH: usize> Drop for BabDigest<WIDTH> {
    fn drop(&mut self) {
        self.0.zeroize();
    }
}
