/* Copyright (c) [2023] [Syswonder Community]
 *   [sysHyper-t1] is licensed under Mulan PSL v2.
 *   You can use this software according to the terms and conditions of the Mulan PSL v2.
 *   You may obtain a copy of Mulan PSL v2 at:
 *               http://license.coscl.org.cn/MulanPSL2
 *   THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
 *   See the Mulan PSL v2 for more details.
 */

use core::{alloc::Layout, mem::size_of};

use crate::config::KERNEL_HEAP_SIZE;
use buddy_system_allocator::LockedHeap;

#[global_allocator]
static HEAP_ALLOCATOR: LockedHeap<32> = LockedHeap::empty();

#[alloc_error_handler]
fn handle_alloc_error(layout: Layout) -> ! {
    panic!("Heap allocation error, layout = {:?}", layout);
}

static mut HEAP_SPACE: [u64; KERNEL_HEAP_SIZE / size_of::<u64>()] =
    [0; KERNEL_HEAP_SIZE / size_of::<u64>()];

pub(super) fn init() {
    let heap_start = unsafe { HEAP_SPACE.as_ptr() as usize };
    println!(
        "Initializing heap at: [{:#x}, {:#x})",
        heap_start,
        heap_start + KERNEL_HEAP_SIZE
    );
    unsafe { HEAP_ALLOCATOR.lock().init(heap_start, KERNEL_HEAP_SIZE) }
}
