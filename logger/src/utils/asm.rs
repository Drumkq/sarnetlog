use std::ffi::c_void;

/// Returns the absolute address at jmp instruction (E8)
/// # Safety
/// Unsafe
pub const unsafe fn get_absolute_address(
    target: *const c_void,
    instruction_sz: isize,
) -> *const c_void {
    let rva = target
        .byte_offset(instruction_sz)
        .cast::<i32>()
        .read_unaligned();

    let fixed_rva = match rva > std::i32::MAX {
        true => -rva,
        false => rva,
    };

    let next_instruction = target
        .byte_offset(instruction_sz)
        .byte_offset(size_of::<i32>() as isize);

    if rva > std::i32::MAX {
        next_instruction.wrapping_sub(fixed_rva as usize)
    } else {
        next_instruction.wrapping_add(fixed_rva as usize)
    }
}
