/* Copyright (c) [2023] [Syswonder Community]
 *   [sysHyper-t1] is licensed under Mulan PSL v2.
 *   You can use this software according to the terms and conditions of the Mulan PSL v2.
 *   You may obtain a copy of Mulan PSL v2 at:
 *               http://license.coscl.org.cn/MulanPSL2
 *   THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
 *   See the Mulan PSL v2 for more details.
 */

use core::time::Duration;

use crate::arch::timer;

pub type TimeValue = Duration;

pub const NANOS_PER_SEC: u64 = 1_000_000_000;

pub fn current_time() -> TimeValue {
    TimeValue::from_nanos(timer::ticks_to_nanos(timer::current_ticks()))
}
