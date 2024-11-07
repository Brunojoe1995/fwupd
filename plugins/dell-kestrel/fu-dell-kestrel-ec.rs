/*
 * Copyright 2024 Dell Technologies
 *
 * SPDX-License-Identifier: LGPL-2.1-or-later OR MIT
 */

#[repr(u8)] // EC USB HID host command
enum FuDellKestrelEcHidCmd {
    SetDockPkg = 0x01,
    GetDockInfo = 0x02,
    GetDockData = 0x03,
    GetDockType = 0x05,
    SetModifyLock = 0x0a,
    SetFwupMode = 0x0b,
    SetPassive = 0x0d,
}

#[repr(u8)] // FuDellKestrelDockInfoStructure::FuDellKestrelEcAddrMap::location
enum FuDellKestrelEcLocation {
    Base = 0x00,
    Module,
}

#[repr(u8)] // FuDellKestrelDockInfoStructure::FuDellKestrelEcAddrMap::device_type
enum FuDellKestrelEcDevType{
    MainEc = 0x00,
    Pd,
    Usbhub,
    Mst,
    Tbt,
    Qi,
    DpMux,
    Lan,
    Fan,
    Rmm,
    Wtpd,
}

#[repr(u8)] // subtype to FuDellKestrelEcDevType::Usbhub
enum FuDellKestrelEcDevUsbhubSubtype {
    Rts0 = 0,
    Rts5,
}

#[repr(u8)] // subtype to FuDellKestrelEcDevType::Tbt
enum FuDellKestrelEcDevTbtSubtype {
    Tr = 0,
    Gr,
    Br,
}

#[repr(u8)] // subtype to FuDellKestrelEcDevType::Mst
enum FuDellKestrelEcDevMstSubtype {
    Vmm8 = 0,
    Vmm9,
}

#[repr(u8)] // subtype to FuDellKestrelEcDevType::Pd
enum FuDellKestrelEcDevPdSubtype {
    Ti = 0,
}

#[repr(u8)] // instance to EcDockDevicePdSubtype::Ti
enum FuDellKestrelEcDevPdSubtypeTiInstance {
    Up5 = 0,
    Up15,
    Up17,
}

#[repr(u8)] // private enum for dock sku
enum FuDellKestrelDockSku {
    Dpalt = 0x01,
    T4,
    T5,
}

#[repr(u8)] // dock resp to chunk write
enum FuDellKestrelEcRespToChunk {
    UpdateComplete = 1,
    SendNextChunk,
    UpdateFailed,
}

#[repr(C, packed)]
#[derive(New, Getters, Parse)]
struct FuStructDellKestrelDockData {
    dock_configuration: u8,
    dock_type: u8,
    reserved: u16,
    module_type: u16,
    reserved: u16,
    reserved: u16,
    reserved: u16,
    dock_firmware_pkg_ver: u32,
    module_serial: u64,
    reserved: u64,
    service_tag: [char; 7],
    marketing_name: [char; 32],
    reserved: u32,
    reserved: u32,
    reserved: u32,
    reserved: u8,
    dock_status: u32,
    reserved: u16,
    reserved: u16,
    dock_mac_addr: [u8; 6],
    reserved: u32,
    reserved: u32,
    reserved: u32,
    reserved: u32,
    reserved: u16,
    eppid: u8,
    reserved: [u8; 74],
}

#[repr(C, packed)]
struct FuStructDellKestrelPackageFwVersions {
	pkg_ver: u32le,
	ec_ver: u32le,
	mst_ver: u32le,
	rts0_g2_ver: u32le,
	rts5_g2_ver: u32le,
	rts0_g1_ver: u32le,
	tbt_ver: u32le,
	pd_up5_ver: u32le,
	pd_up15_ver: u32le,
	pd_up17_ver: u32le,
	dpmux_ver: u32le,
	rmm_ver: u32le,
	lan_ver: u32le,
	reserved: [u32le; 3],
}
