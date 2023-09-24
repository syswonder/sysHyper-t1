/* Copyright (c) [2023] [Syswonder Community]
 *   [sysHyper-t1] is licensed under Mulan PSL v2.
 *   You can use this software according to the terms and conditions of the Mulan PSL v2.
 *   You may obtain a copy of Mulan PSL v2 at:
 *               http://license.coscl.org.cn/MulanPSL2
 *   THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
 *   See the Mulan PSL v2 for more details.
 */

use core::arch::global_asm;

use aarch64_cpu::registers::*;
use tock_registers::interfaces::Writeable;

global_asm!(include_str!("trap.S"));

pub fn init() {
    extern "C" {
        fn exception_vector_base();
    }
    VBAR_EL2.set(exception_vector_base as usize as _);
}