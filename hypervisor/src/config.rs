/* Copyright (c) [2023] [Syswonder Community]
 *   [sysHyper-t1] is licensed under Mulan PSL v2.
 *   You can use this software according to the terms and conditions of the Mulan PSL v2.
 *   You may obtain a copy of Mulan PSL v2 at:
 *               http://license.coscl.org.cn/MulanPSL2
 *   THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
 *   See the Mulan PSL v2 for more details.
 */

use core::sync::atomic::AtomicUsize;

pub const BOOT_KERNEL_STACK_SIZE: usize = 0x4000;

pub const KERNEL_HEAP_SIZE: usize = 0x40_0000;

pub const PHYS_VIRT_OFFSET: usize = 0x4000_0000;
pub const PHYS_MEMORY_END: usize = 0x6000_0000;

pub const CPU_NUM: usize = 4;
pub const VM_NUM: usize = 3;
pub const CPU_TO_VM: [usize; CPU_NUM] = [0, 1, 2, 2];

pub const PRIMARY_CPU_ID: usize = 0;

pub const BLK_QUEUE_SIZE: usize = 16;

const ATOMIC_ZERO: AtomicUsize = AtomicUsize::new(0);
pub static mut GUEST_ENTRIES: [AtomicUsize; CPU_NUM] = [ATOMIC_ZERO; CPU_NUM];
pub static mut PSCI_CONTEXT: [AtomicUsize; CPU_NUM] = [ATOMIC_ZERO; CPU_NUM];
