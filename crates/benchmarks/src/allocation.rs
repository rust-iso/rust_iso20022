//! Process-local requested-allocation counters for benchmark subprocesses.

use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicU64, Ordering};

pub struct CountingAllocator {
    calls: AtomicU64,
    allocated_bytes: AtomicU64,
    deallocated_bytes: AtomicU64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AllocationSnapshot {
    pub calls: u64,
    pub allocated_bytes: u64,
    pub deallocated_bytes: u64,
    pub live_requested_bytes: u64,
}

impl CountingAllocator {
    pub const fn new() -> Self {
        Self {
            calls: AtomicU64::new(0),
            allocated_bytes: AtomicU64::new(0),
            deallocated_bytes: AtomicU64::new(0),
        }
    }

    pub fn reset(&self) {
        self.calls.store(0, Ordering::Relaxed);
        self.allocated_bytes.store(0, Ordering::Relaxed);
        self.deallocated_bytes.store(0, Ordering::Relaxed);
    }

    pub fn snapshot(&self) -> AllocationSnapshot {
        let allocated_bytes = self.allocated_bytes.load(Ordering::Relaxed);
        let deallocated_bytes = self.deallocated_bytes.load(Ordering::Relaxed);
        AllocationSnapshot {
            calls: self.calls.load(Ordering::Relaxed),
            allocated_bytes,
            deallocated_bytes,
            live_requested_bytes: allocated_bytes.saturating_sub(deallocated_bytes),
        }
    }

    fn record_allocation(&self, bytes: usize) {
        self.calls.fetch_add(1, Ordering::Relaxed);
        self.allocated_bytes
            .fetch_add(bytes as u64, Ordering::Relaxed);
    }
}

impl Default for CountingAllocator {
    fn default() -> Self {
        Self::new()
    }
}

// SAFETY: every operation delegates to `System` with the original pointer and
// layout. The additional atomics do not access allocated memory or alter ABI.
unsafe impl GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // SAFETY: delegated with the caller-provided valid layout.
        let pointer = unsafe { System.alloc(layout) };
        if !pointer.is_null() {
            self.record_allocation(layout.size());
        }
        pointer
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        // SAFETY: delegated with the caller-provided valid layout.
        let pointer = unsafe { System.alloc_zeroed(layout) };
        if !pointer.is_null() {
            self.record_allocation(layout.size());
        }
        pointer
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        self.deallocated_bytes
            .fetch_add(layout.size() as u64, Ordering::Relaxed);
        // SAFETY: delegated with the same pointer/layout contract as the caller.
        unsafe { System.dealloc(pointer, layout) };
    }

    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        // SAFETY: delegated with the caller-provided pointer/layout/new size.
        let replacement = unsafe { System.realloc(pointer, layout, new_size) };
        if !replacement.is_null() {
            self.deallocated_bytes
                .fetch_add(layout.size() as u64, Ordering::Relaxed);
            self.record_allocation(new_size);
        }
        replacement
    }
}
