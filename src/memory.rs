use x86_64::{
    structures::paging::PageTable,
    VirtAddr,
};

/// Returns a mutable reference to the active level 4 table.
///
/// This function is `unsafe` because the caller must guarantee that the
/// completep hysical memory is mapped to virtual memory at the passed
/// `physical_memory_offset`. Also, this function must be only called once to
/// avoid aliasing `&mut` references (which is undefined behaviour).
pub unsafe fn active_level_4_table(physical_memory_offset: VirtAddr)
                                   -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    // Read the active l4 table
    let (level_4_table_frame, _) = Cr3::read();

    // Obtain start addr of active l4 table
    let phys = level_4_table_frame.start_address();

    // Obtain addr of physical memory
    let virt = physical_memory_offset + phys.as_u64();

    // Deref indirection to obtain raw memory mapping
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    &mut *page_table_ptr
}
