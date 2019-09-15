#[allow(deprecated)]
use hash::{Hasher, SipHasher13};

/// The default [`Hasher`] used by [`RandomState`].
///
/// The internal algorithm is not specified, and so it and its hashes should
/// not be relied upon over releases.
///
/// [`RandomState`]: struct.RandomState.html
/// [`Hasher`]: ../../hash/trait.Hasher.html
#[stable(feature = "hashmap_default_hasher", since = "1.13.0")]
#[allow(deprecated)]
#[derive(Clone)]
pub struct DefaultHasher(SipHasher13);

impl DefaultHasher {
    /// Creates a new `DefaultHasher`.
    ///
    /// This hasher is not guaranteed to be the same as all other
    /// `DefaultHasher` instances, but is the same as all other `DefaultHasher`
    /// instances created through `new` or `default`.
    #[stable(feature = "hashmap_default_hasher", since = "1.13.0")]
    #[allow(deprecated)]
    pub fn new() -> DefaultHasher {
        DefaultHasher(SipHasher13::new_with_keys(0, 0))
    }
}

#[stable(feature = "hashmap_default_hasher", since = "1.13.0")]
impl Default for DefaultHasher {
    // FIXME: here should link `new` to [DefaultHasher::new], but it occurs intra-doc link
    // resolution failure when re-exporting libstd items. When #56922 fixed,
    // link `new` to [DefaultHasher::new] again.
    /// Creates a new `DefaultHasher` using `new`.
    /// See its documentation for more.
    fn default() -> DefaultHasher {
        DefaultHasher::new()
    }
}

#[stable(feature = "hashmap_default_hasher", since = "1.13.0")]
impl Hasher for DefaultHasher {
    #[inline]
    fn finish(&self) -> u64 {
        self.0.finish()
    }

    #[inline]
    fn write(&mut self, msg: &[u8]) {
        self.0.write(msg)
    }
}

