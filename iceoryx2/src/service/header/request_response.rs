// Copyright (c) 2025 Contributors to the Eclipse Foundation
//
// See the NOTICE file(s) distributed with this work for additional
// information regarding copyright ownership.
//
// This program and the accompanying materials are made available under the
// terms of the Apache Software License 2.0 which is available at
// https://www.apache.org/licenses/LICENSE-2.0, or the MIT license
// which is available at https://opensource.org/licenses/MIT.
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use iceoryx2_bb_derive_macros::ZeroCopySend;

use crate::port::port_identifiers::{UniqueClientId, UniqueServerId};

/// Request header used by
/// [`MessagingPattern::RequestResponse`](crate::service::messaging_pattern::MessagingPattern::RequestResponse)
#[derive(Debug, Copy, Clone, ZeroCopySend)]
#[repr(C)]
pub struct RequestHeader {
    pub(crate) client_port_id: UniqueClientId,
}

impl RequestHeader {
    /// Returns the [`UniqueClientId`] of the [`Client`](crate::port::client::Client)
    /// which sent the [`RequestMut`](crate::request_mut::RequestMut)
    pub fn client_port_id(&self) -> UniqueClientId {
        self.client_port_id
    }
}

/// Response header used by
/// [`MessagingPattern::RequestResponse`](crate::service::messaging_pattern::MessagingPattern::RequestResponse)
#[derive(Debug, Copy, Clone, ZeroCopySend)]
#[repr(C)]
pub struct ResponseHeader {
    pub(crate) server_port_id: UniqueServerId,
}

impl ResponseHeader {
    /// Returns the [`UniqueServerId`] of the [`Server`](crate::port::server::Server)
    /// which sent the [`Response`](crate::response::Response)
    pub fn server_port_id(&self) -> UniqueServerId {
        self.server_port_id
    }
}
