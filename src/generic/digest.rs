#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
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
