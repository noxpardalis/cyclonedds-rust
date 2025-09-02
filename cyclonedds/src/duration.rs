// TODO(noxpardalis): it would probably be better to make this its own full-fat type to better
// map the semantics of duration::infinite etc...
/// A `Duration` represents a relative span of time. This is typically used in
/// DDS to represent timeouts.
pub type Duration = std::time::Duration;
