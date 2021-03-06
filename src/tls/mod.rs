/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

macro_rules! checklock (
    ($e: expr) => {
        match $e {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        }
    }
);

mod certificate_manager;
mod certificate_record;
mod https_server_factory;
mod ssl_context;

pub use tls::certificate_manager::*;
pub use tls::certificate_record::*;
pub use tls::https_server_factory::*;
pub use tls::ssl_context::*;


#[derive(Clone, Eq, PartialEq)]
pub enum TlsOption {
    Enabled,
    Disabled,
}
