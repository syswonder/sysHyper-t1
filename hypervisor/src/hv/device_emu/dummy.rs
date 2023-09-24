/* Copyright (c) [2023] [Syswonder Community]
 *   [sysHyper-t1] is licensed under Mulan PSL v2.
 *   You can use this software according to the terms and conditions of the Mulan PSL v2.
 *   You may obtain a copy of Mulan PSL v2 at:
 *               http://license.coscl.org.cn/MulanPSL2
 *   THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
 *   See the Mulan PSL v2 for more details.
 */

use rvm::RvmResult;

use crate::hv::gpm::GuestPhysMemorySet;

use super::MMIODevice;

pub struct Dummy {
    base_vaddr: usize,
    dummy_size: usize,
}

impl Dummy {
    pub const fn new(base_vaddr: usize, dummy_size: usize) -> Self {
        Self {
            base_vaddr,
            dummy_size,
        }
    }
}

impl MMIODevice for Dummy {
    fn mem_range(&self) -> core::ops::Range<usize> {
        self.base_vaddr..self.base_vaddr + self.dummy_size
    }

    fn read(&self, _: usize, _: u8) -> RvmResult<u32> {
        Ok(0)
    }

    fn write(&self, _: usize, _: u32, _: u8, _: &GuestPhysMemorySet) -> RvmResult {
        Ok(())
    }
}
