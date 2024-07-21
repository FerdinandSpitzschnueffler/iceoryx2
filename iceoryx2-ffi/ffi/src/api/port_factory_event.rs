// Copyright (c) 2024 Contributors to the Eclipse Foundation
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

#![allow(non_camel_case_types)]

use crate::api::{iox2_service_type_e, HandleToType};

use iceoryx2::prelude::*;
use iceoryx2::service::port_factory::event::PortFactory;
use iceoryx2_bb_elementary::static_assert::*;
use iceoryx2_ffi_macros::iceoryx2_ffi;

use core::mem::ManuallyDrop;

// BEGIN types definition

pub(super) union PortFactoryEventUnion {
    ipc: ManuallyDrop<PortFactory<zero_copy::Service>>,
    local: ManuallyDrop<PortFactory<process_local::Service>>,
}

impl PortFactoryEventUnion {
    pub(super) fn new_ipc(port_factory: PortFactory<zero_copy::Service>) -> Self {
        Self {
            ipc: ManuallyDrop::new(port_factory),
        }
    }
    pub(super) fn new_local(port_factory: PortFactory<process_local::Service>) -> Self {
        Self {
            local: ManuallyDrop::new(port_factory),
        }
    }
}

#[repr(C)]
#[repr(align(8))] // alignment of Option<PortFactoryEventUnion>
pub struct iox2_port_factory_event_storage_t {
    internal: [u8; 1656], // magic number obtained with size_of::<Option<PortFactoryEventUnion>>()
}

#[repr(C)]
#[iceoryx2_ffi(PortFactoryEventUnion)]
pub struct iox2_port_factory_event_t {
    service_type: iox2_service_type_e,
    value: iox2_port_factory_event_storage_t,
    deleter: fn(*mut iox2_port_factory_event_t),
}

impl iox2_port_factory_event_t {
    pub(super) fn init(
        &mut self,
        service_type: iox2_service_type_e,
        value: PortFactoryEventUnion,
        deleter: fn(*mut iox2_port_factory_event_t),
    ) {
        self.service_type = service_type;
        self.value.init(value);
        self.deleter = deleter;
    }
}

pub struct iox2_port_factory_event_h_t;
/// The owning handle for `iox2_port_factory_event_t`. Passing the handle to an function transfers the ownership.
pub type iox2_port_factory_event_h = *mut iox2_port_factory_event_h_t;

pub struct iox2_port_factory_event_ref_h_t;
/// The non-owning handle for `iox2_port_factory_event_t`. Passing the handle to an function does not transfers the ownership.
pub type iox2_port_factory_event_ref_h = *mut iox2_port_factory_event_ref_h_t;

impl HandleToType for iox2_port_factory_event_h {
    type Target = *mut iox2_port_factory_event_t;

    fn as_type(self) -> Self::Target {
        self as *mut _ as _
    }
}

impl HandleToType for iox2_port_factory_event_ref_h {
    type Target = *mut iox2_port_factory_event_t;

    fn as_type(self) -> Self::Target {
        self as *mut _ as _
    }
}

// END type definition

// BEGIN C API

/// This function needs to be called to destroy the port factory!
///
/// # Arguments
///
/// * `port_factory_handle` - A valid [`iox2_port_factory_event_h`]
///
/// # Safety
///
/// * The `port_factory_handle` is invalid after the return of this function and leads to undefined behavior if used in another function call!
/// * The corresponding [`iox2_port_factory_event_t`] can be re-used with a call to
///   [`iox2_service_builder_event_open_or_create`](crate::iox2_service_builder_event_open_or_create) or
///   [`iox2_service_builder_event_open`](crate::iox2_service_builder_event_open)!
#[no_mangle]
pub unsafe extern "C" fn iox2_port_factory_event_drop(
    port_factory_handle: iox2_port_factory_event_h,
) {
    debug_assert!(!port_factory_handle.is_null());

    let port_factory = &mut *port_factory_handle.as_type();

    match port_factory.service_type {
        iox2_service_type_e::IPC => {
            ManuallyDrop::drop(&mut port_factory.value.as_mut().ipc);
        }
        iox2_service_type_e::LOCAL => {
            ManuallyDrop::drop(&mut port_factory.value.as_mut().local);
        }
    }
    (port_factory.deleter)(port_factory);
}

// END C API
