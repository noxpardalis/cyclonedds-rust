use md5::Digest;

use crate::cdr_bounds::{CdrBounds, CdrSize};

///
pub struct KeyHash(pub(crate) [u8; 16]);

impl KeyHash {
    ///
    pub fn from_key<T>(key: &T::Key, force_md5: bool) -> Option<KeyHash>
    where
        T: crate::Topicable,
    {
        let mut serialized = cdr_encoding::to_vec::<_, byteorder::BigEndian>(&key).ok()?;

        let max_possible_serialized_size = T::Key::max_serialized_cdr_size();

        let key_hash = if force_md5 || max_possible_serialized_size > CdrSize::Bounded(16) {
            // The key hash should be computed via MD5.
            let mut hasher = md5::Md5::new();
            hasher.update(serialized);
            let hash = hasher.finalize();
            hash.into()
        } else {
            // The CDR serialized form fits and can be used as the key hash but
            // it must be padded to 16 bytes and those padding bytes must be zeroed.
            serialized.resize(16, 0);
            // TODO could this possibly fail? If not the whole function can
            // return just a KeyHash and not an Option.
            serialized.try_into().ok()?
        };

        Some(KeyHash(key_hash))
    }
}
