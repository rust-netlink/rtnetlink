// SPDX-License-Identifier: MIT

//! Traffic control manipulation utilities.
//! See [`tc`].
//!
//! [`tc`]: https://man7.org/linux/man-pages/man8/tc.8.html

pub use self::add_action::*;
pub use self::add_filter::*;
pub use self::add_qdisc::*;
pub use self::del_action::*;
pub use self::del_qdisc::*;
pub use self::get::*;
pub use self::handle::*;

mod handle;

mod get;

mod add_qdisc;

mod del_qdisc;

mod add_filter;

mod del_action;

mod add_action;

#[cfg(test)]
mod test;
