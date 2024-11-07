/*
 * Copyright 2024 Dell Technologies
 *
 * SPDX-License-Identifier: LGPL-2.1-or-later OR MIT
 */

#[repr(u8)]
enum FuDellDockBaseType {
    Unknown = 0x0,
    Kestrel = 0x07,
}

#[repr(u8)] // EcV2HidCmd::SetModifyLock
enum FuDellKestrelEcDevLock {
    Ec = 0x01,
    UsbHubPrimary = 0x07,
    UsbHubSecondary = 0x08,
    MstHub = 0x09,
    Tbt = 0x0a,
}
