use crate::{WIDTH, generic::BabDigest};

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
#[repr(transparent)]
pub struct William3Digest(pub(crate) BabDigest<WIDTH>);

impl From<BabDigest<WIDTH>> for William3Digest {
    fn from(value: BabDigest<WIDTH>) -> Self {
        Self(value)
    }
}

impl From<[u8; WIDTH]> for William3Digest {
    fn from(value: [u8; WIDTH]) -> Self {
        value.into()
    }
}

impl From<William3Digest> for BabDigest<WIDTH> {
    fn from(value: William3Digest) -> Self {
        value.0
    }
}

impl From<William3Digest> for [u8; WIDTH] {
    fn from(value: William3Digest) -> Self {
        value.0.into()
    }
}
