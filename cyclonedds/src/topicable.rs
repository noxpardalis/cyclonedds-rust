pub trait Topicable:
    serde::ser::Serialize + serde::de::DeserializeOwned + std::clone::Clone + std::fmt::Debug
{
    type Key: serde::ser::Serialize
        + serde::de::DeserializeOwned
        + std::cmp::PartialEq
        + std::hash::Hash
        + std::default::Default
        + std::clone::Clone
        + std::fmt::Debug
        + crate::cdr_bounds::CdrBounds;

    // TODO decide if this is exposed in the trait.
    ///
    const IS_KEYED: bool = std::mem::size_of::<Self::Key>() != 0;

    // TODO decide if this is exposed in the trait and if this is the right way
    // to expose the MD5_KEYHASH knob.
    ///
    const FORCE_MD5_KEYHASH: bool = false;

    // TODO decide on name here. Does `from` imply move of Self::Key?
    ///
    fn from_key(key: &Self::Key) -> Self;

    ///
    fn as_key(&self) -> Self::Key;

    ///
    fn type_name() -> impl AsRef<str> {
        let full_type_path = std::any::type_name::<Self>();

        // Strip out the leading module if it exists.
        if let Some((_, type_path)) = full_type_path.split_once("::") {
            type_path
        } else {
            // There is no leading module so the full type path is just the type name.
            full_type_path
        }
    }
}
