/* Copyright (c) [2023] [Syswonder Community]
 *   [sysHyper-t1] is licensed under Mulan PSL v2.
 *   You can use this software according to the terms and conditions of the Mulan PSL v2.
 *   You may obtain a copy of Mulan PSL v2 at:
 *               http://license.coscl.org.cn/MulanPSL2
 *   THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
 *   See the Mulan PSL v2 for more details.
 */

use rvm::RvmResult;
use spin::Mutex;

use crate::hv::gpm::GuestPhysMemorySet;

use super::MMIODevice;

pub struct Vgic {
    base_vaddr: usize,
    // TODO
    inner: Mutex<VgicdInner>,
}

#[derive(Default)]
pub struct VgicdInner {
    enabled: bool,
}

impl VgicdInner {
    pub const fn new() -> Self {
        Self { enabled: false }
    }
}

// TODO: merge these consts with regs in gicv2.rs.
const GICD_CTLR: usize = 0x00;
const GICD_TYPER: usize = 0x04;
const GICD_IIDR: usize = 0x08;

// copy from gicv2.rs.
impl Vgic {
    pub const fn new(base_vaddr: usize) -> Self {
        Self {
            base_vaddr,
            inner: Mutex::new(VgicdInner::new()),
        }
    }
}

impl MMIODevice for Vgic {
    fn mem_range(&self) -> core::ops::Range<usize> {
        self.base_vaddr..self.base_vaddr + 0x10000
    }

    fn read(&self, addr: usize, access_size: u8) -> RvmResult<u32> {
        // TODO: read SGI-related registers
        trace!("GICD read addr 0x{:x}, access size {}", addr, access_size);
        let val = match addr - self.base_vaddr {
            _ => unsafe { (addr as *const u32).read_volatile() },
        };
        Ok(val)
    }

    fn write(&self, addr: usize, val: u32, access_size: u8, _: &GuestPhysMemorySet) -> RvmResult {
        trace!("GICD write addr 0x{:x}, access size {}", addr, access_size);
        // TODO: write SGI
        Ok(())
    }
}
