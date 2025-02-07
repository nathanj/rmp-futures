#![feature(async_await)]
// async syntax confuses clippy (2019/07/30)
#![allow(clippy::needless_lifetimes)]

pub mod decode;
pub mod encode;
pub mod rpc;

/// Used when iterating over collections, to return either the next item or
/// indicate end of the collection, returning the underlying reader.
pub enum MsgPackOption<T, U> {
    Some(T),
    End(U),
}

impl<T, U> MsgPackOption<T, U> {
    /// Convert to an `Option`, dropping U
    pub fn into_option(self) -> Option<T> {
        match self {
            MsgPackOption::Some(t) => Some(t),
            MsgPackOption::End(_u) => None,
        }
    }

    pub fn unwrap(self) -> T {
        self.into_option().unwrap()
    }

    pub fn unwrap_end(self) -> U {
        if let MsgPackOption::End(u) = self {
            Some(u)
        } else {
            None
        }
        .unwrap()
    }
}
