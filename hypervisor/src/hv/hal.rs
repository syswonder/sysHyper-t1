/* Copyright (c) [2023] [Syswonder Community]
 *   [sysHyper-t1] is licensed under Mulan PSL v2.
 *   You can use this software according to the terms and conditions of the Mulan PSL v2.
 *   You may obtain a copy of Mulan PSL v2 at:
 *               http://license.coscl.org.cn/MulanPSL2
 *   THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
 *   See the Mulan PSL v2 for more details.
 */

use rvm::{HostPhysAddr, HostVirtAddr, RvmHal, RvmVcpu};

use crate::mm::{address, frame};

use super::vmexit;

#[derive(Debug)]
pub struct RvmHalImpl;

impl RvmHal for RvmHalImpl {
    fn alloc_page() -> Option<HostPhysAddr> {
        unsafe { frame::alloc_page() }
    }

    fn dealloc_page(paddr: HostPhysAddr) {
        unsafe { frame::dealloc_page(paddr) }
    }

    fn phys_to_virt(paddr: HostPhysAddr) -> HostVirtAddr {
        address::phys_to_virt(paddr)
    }

    fn virt_to_phys(vaddr: HostVirtAddr) -> HostPhysAddr {
        address::virt_to_phys(vaddr)
    }

    fn vmexit_handler(vcpu: &mut RvmVcpu<Self>) {
        vmexit::vmexit_handler(vcpu);
    }
}
