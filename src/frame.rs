//! Functionality for managing physical frames.

use x86_64::PhysAddr;
use x86_64::structures::paging::{FrameAllocator, PhysFrame, Size4KiB};
use bootloader_precompiled::bootinfo::{MemoryMap, MemoryRegion, MemoryRegionType};

/// A simple frame allocator.
///
/// TODO: Implement frame deallocation.
pub struct Allocator<'a> {
    memory_map: &'a MemoryMap,
    cur_region_idx: Option<usize>,
    next_frame: u64,
}

impl<'a> Allocator<'a> {
    pub const FRAME_SIZE: u64 = 4096;

    pub fn new(memory_map: &'a MemoryMap) -> Allocator<'a> {
        let mut this = Allocator {
            memory_map,
            cur_region_idx: None,
            next_frame: 0,
        };
        this.find_next_usable_region();
        this
    }
}

impl<'a> FrameAllocator<Size4KiB> for Allocator<'a> {
    fn alloc(&mut self) -> Option<PhysFrame> {
        self.ensure_next_frame();

        self.cur_region().map(|region| {
            let addr = Self::FRAME_SIZE * self.next_frame;
            assert!(addr >= region.range.start_addr());
            assert!(addr < region.range.end_addr());

            self.next_frame += 1;
            PhysFrame::from_start_address(PhysAddr::new(addr)).unwrap()
        })
    }
}

impl<'a> Allocator<'a> {
    fn cur_region(&self) -> Option<&'a MemoryRegion> {
        self.cur_region_idx.map(|i| &self.memory_map[i])
    }

    /// If `self.next_frame` is beyond the current region, advance to the next.
    fn ensure_next_frame(&mut self) {
        if let Some(region) = self.cur_region() {
            if self.next_frame >= region.range.end_frame_number {
                self.find_next_usable_region();
            }
        }
    }

    /// Updates the current region to the next Usable region if there is one, or None.
    fn find_next_usable_region(&mut self) {
        // cur_region_idx is only None at init time.
        let next_idx = self.cur_region_idx.map(|x| x + 1).unwrap_or(0);

        for idx in next_idx..self.memory_map.len() {
            if self.memory_map[idx].region_type == MemoryRegionType::Usable {
                self.cur_region_idx = Some(idx);
                self.next_frame = self.memory_map[idx].range.start_frame_number;
                return;
            }
        }
        self.cur_region_idx = None;
    }
}

#[cfg(test)]
mod test {
    extern crate std;
    use std::boxed::Box;
    use std::borrow::Borrow;

    use super::*;
    use bootloader_precompiled::bootinfo::{MemoryMap, MemoryRegion, MemoryRegionType, FrameRange};
    use x86_64::structures::paging::PhysFrame;

    fn region(start_addr: u64, end_addr: u64, region_type: MemoryRegionType) -> MemoryRegion {
        MemoryRegion {
            range: FrameRange::new(start_addr, end_addr),
            region_type
        }
    }

    fn frame(start_addr: u64) -> PhysFrame {
        PhysFrame::from_start_address(PhysAddr::new(start_addr)).unwrap()
    }

    #[test]
    fn test() {
        use self::MemoryRegionType::*;

        let mut map = Box::new(MemoryMap::new());
        map.add_region(region(0x0000_0000, 0x0000_1000, FrameZero));
        map.add_region(region(0x0000_1000, 0x0000_5000, PageTable));
        map.add_region(region(0x0000_5000, 0x0001_7000, Bootloader));
        map.add_region(region(0x0002_0000, 0x0002_4000, Usable));
        map.add_region(region(0x0003_0000, 0x0004_0000, Reserved));
        map.add_region(region(0x0005_0000, 0x0006_0000, Reserved));
        map.add_region(region(0x0006_0000, 0x0006_3000, Usable));

        let mut allocator = Allocator::new(map.borrow());
        assert_eq!(Some(frame(0x0002_0000)), allocator.alloc());
        assert_eq!(Some(frame(0x0002_1000)), allocator.alloc());
        assert_eq!(Some(frame(0x0002_2000)), allocator.alloc());
        assert_eq!(Some(frame(0x0002_3000)), allocator.alloc());
        assert_eq!(Some(frame(0x0006_0000)), allocator.alloc());
        assert_eq!(Some(frame(0x0006_1000)), allocator.alloc());
        assert_eq!(Some(frame(0x0006_2000)), allocator.alloc());
        assert_eq!(None, allocator.alloc());
        assert_eq!(None, allocator.alloc());
    }
}
