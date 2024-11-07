/*
 * Copyright 2024 Dell Technologies
 *
 * SPDX-License-Identifier: LGPL-2.1-or-later OR MIT
 */

#[repr(u8)]
enum FuDellKestrelEcUsbHidCmd {
    WriteData = 0x40,
    ExtI2cWrite = 0xc6,
    ExtI2cRead = 0xd6,
}

#[repr(C, packed)]
#[derive(New, Setters, Getters, Default)]
struct FuStructEcHidCmdBuffer {
    cmd: u8,
    ext: u8,
    dwregaddr: u32,
    bufferlen: u16,
    parameters: [u8; 3] = 0xEC0180, // addr, length, speed
    extended_cmdarea: [u8; 53] = 0x00,
    databytes: [u8; 192] = 0x00,
}
