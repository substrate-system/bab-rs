use crate::{WIDTH, generic::BabDigest};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
#[repr(transparent)]
pub struct William3Digest(pub(crate) BabDigest<WIDTH>);

impl William3Digest {
    /// Converts `self` into the underlying byte array.
    ///
    /// This type deliberately does not provide this functionality through a trait, in order to make it less likely to leak values secrets.
    pub fn into_bytes(self) -> [u8; WIDTH] {
        self.0.into_bytes()
    }

    /// Returns a reference to the underlying byte array.
    ///
    /// This type deliberately does not provide this functionality through a trait, in order to make it less likely to leak values secrets.
    pub fn as_bytes(&self) -> &[u8; WIDTH] {
        self.0.as_bytes()
    }

    /// Returns a mutable reference to the underlying byte array.
    ///
    /// This type deliberately does not provide this functionality through a trait, in order to make it less likely to leak values secrets.
    pub fn as_mut_bytes(&mut self) -> &mut [u8; WIDTH] {
        self.0.as_mut_bytes()
    }
}

impl From<BabDigest<WIDTH>> for William3Digest {
    fn from(value: BabDigest<WIDTH>) -> Self {
        Self(value)
    }
}

impl From<[u8; WIDTH]> for William3Digest {
    fn from(value: [u8; WIDTH]) -> Self {
        Self(value.into())
    }
}

impl From<William3Digest> for BabDigest<WIDTH> {
    fn from(value: William3Digest) -> Self {
        value.0
    }
}

impl Default for William3Digest {
    fn default() -> Self {
        [0; WIDTH].into()
    }
}
