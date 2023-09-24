/* Copyright (c) [2023] [Syswonder Community]
 *   [sysHyper-t1] is licensed under Mulan PSL v2.
 *   You can use this software according to the terms and conditions of the Mulan PSL v2.
 *   You may obtain a copy of Mulan PSL v2 at:
 *               http://license.coscl.org.cn/MulanPSL2
 *   THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
 *   See the Mulan PSL v2 for more details.
 */

use crate::{HostPhysAddr, HostVirtAddr};

pub trait RvmHal: Sized {
    /// Allocates a 4K-sized contiguous physical page, returns its physical address.
    fn alloc_page() -> Option<HostPhysAddr>;
    /// Deallocates the given physical page.
    fn dealloc_page(paddr: HostPhysAddr);
    /// Converts a physical address to a virtual address which can access.
    fn phys_to_virt(paddr: HostPhysAddr) -> HostVirtAddr;
    /// Converts a virtual address to the corresponding physical address.
    fn virt_to_phys(vaddr: HostVirtAddr) -> HostPhysAddr;
    /// VM-Exit handler.
    fn vmexit_handler(vcpu: &mut crate::RvmVcpu<Self>);
}
