use bytes::BytesMut;

use lber::write;
use lber::structures::ASNTag;
use filter::parse;
use super::{MakeCritical, RawControl};

pub const ASSERTION_OID: &'static str = "1.3.6.1.1.12";

/// Assertion control ([RFC 4528](https://tools.ietf.org/html/rfc4528)).
#[derive(Debug)]
pub struct Assertion<S> {
    /// String representation of the assertion filter.
    pub filter: S,
}

impl<S: AsRef<str>> Assertion<S> {
    /// Create a new control instance with the specified filter.
    // RawControl is returned in order to avoid an into() at the call site
    #[cfg_attr(feature="cargo-clippy", allow(new_ret_no_self))]
    pub fn new(filter: S) -> RawControl {
        Assertion { filter: filter }.into()
    }
}

impl<S> MakeCritical for Assertion<S> { }

impl<S: AsRef<str>> From<Assertion<S>> for RawControl {
    fn from(assn: Assertion<S>) -> RawControl {
        let filter_ref = assn.filter.as_ref();
        let filter = parse(filter_ref).expect("filter").into_structure();
        let mut buf = BytesMut::with_capacity(filter_ref.len());    // ballpark
        write::encode_into(&mut buf, filter).expect("encoded");
        RawControl {
            ctype: ASSERTION_OID.to_owned(),
            crit: false,
            val: Some(Vec::from(&buf[..])),
        }
    }
}
