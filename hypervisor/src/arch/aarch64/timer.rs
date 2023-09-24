/* Copyright (c) [2023] [Syswonder Community]
 *   [sysHyper-t1] is licensed under Mulan PSL v2.
 *   You can use this software according to the terms and conditions of the Mulan PSL v2.
 *   You may obtain a copy of Mulan PSL v2 at:
 *               http://license.coscl.org.cn/MulanPSL2
 *   THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
 *   See the Mulan PSL v2 for more details.
 */

use aarch64_cpu::registers::{CNTFRQ_EL0, CNTPCT_EL0};
use tock_registers::interfaces::Readable;

use crate::timer::NANOS_PER_SEC;

pub fn current_ticks() -> u64 {
    return CNTPCT_EL0.get();
}

pub fn ticks_to_nanos(ticks: u64) -> u64 {
    return ticks * NANOS_PER_SEC / CNTFRQ_EL0.get();
}
