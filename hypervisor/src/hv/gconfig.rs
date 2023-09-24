/* Copyright (c) [2023] [Syswonder Community]
 *   [sysHyper-t1] is licensed under Mulan PSL v2.
 *   You can use this software according to the terms and conditions of the Mulan PSL v2.
 *   You may obtain a copy of Mulan PSL v2 at:
 *               http://license.coscl.org.cn/MulanPSL2
 *   THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
 *   See the Mulan PSL v2 for more details.
 */

use rvm::{GuestPhysAddr, HostPhysAddr};
use spin::Mutex;

use crate::config::CPU_NUM;

use super::gpm::GuestPhysMemorySet;

pub const GUEST_IMAGE_PADDR: HostPhysAddr = 0x400_1000;
pub const GUEST_IMAGE_SIZE: usize = 0x10_0000; // 1M

pub const GUEST_PHYS_MEMORY_BASE: GuestPhysAddr = 0x4000_0000;
pub const BIOS_ENTRY: GuestPhysAddr = 0x8000;
pub const GUEST_ENTRY: GuestPhysAddr = 0x4008_0000;
pub const GUEST_PHYS_MEMORY_SIZE: usize = 0x800_0000; // 128M

pub const VIRTIO_HEADER_TOTAL_SIZE: usize = 0x4000;
pub const VIRTIO_HEADER_EACH_SIZE: usize = 0x200;

pub const VIRTIO_BLK_IDX: usize = 0;
pub const VIRTIO_NET_IDX: usize = 1;

pub const GUEST_INITRAMFS_START: usize = 0x4800_0000;
pub const GUEST_INITRAMFS_END: usize = 0x4820_0000;

pub const GUEST_DTB_ADDR: usize = 0x4a00_0000;

#[link_section = ".dtb"]
pub static GUEST_DTB: [u8; include_bytes!("../../../dts/linux_guest.dtb").len()] =
    *include_bytes!("../../../dts/linux_guest.dtb");

#[link_section = ".initramfs"]
pub static GUEST_INITRAMFS: [u8; include_bytes!("../../../bin/initramfs.cpio.gz").len()] =
    *include_bytes!("../../../bin/initramfs.cpio.gz");

const NONE: Option<GuestPhysMemorySet> = None;
pub static GUEST_GPM: Mutex<[Option<GuestPhysMemorySet>; CPU_NUM]> = Mutex::new([NONE; CPU_NUM]);
