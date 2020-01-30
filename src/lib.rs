#![feature(core_intrinsics)]

pub fn ver1(mut x: u64) -> u64 {
    x += 1;
    x -= 1;
    x
}

pub fn ver2(x: u64) -> u64 {
    x + 1 - 1
}

mod new;
mod old;

pub use new::align_offset as align_offset_new;
pub use old::align_offset as align_offset_old;

#[test]
fn align_offset_weird_strides() {
    #[repr(packed)]
    struct A3(u16, u8);
    struct A4(u32);
    #[repr(packed)]
    struct A5(u32, u8);
    #[repr(packed)]
    struct A6(u32, u16);
    #[repr(packed)]
    struct A7(u32, u16, u8);
    #[repr(packed)]
    struct A8(u32, u32);
    #[repr(packed)]
    struct A9(u32, u32, u8);
    #[repr(packed)]
    struct A10(u32, u32, u16);

    unsafe fn test_weird_stride<T>(ptr: *const T, align: usize) -> bool {
        let numptr = ptr as usize;
        let mut expected = usize::max_value();
        // Naive but definitely correct way to find the *first* aligned element of stride::<T>.
        for el in 0..align {
            if (numptr + el * ::std::mem::size_of::<T>()) % align == 0 {
                expected = el;
                break;
            }
        }
        let got1 = align_offset_old(ptr, align);
        let got2 = align_offset_new(ptr, align);
        let mut ret = false;

        if got1 != expected {
            eprintln!(
                "old: aligning {:p} (with stride of {}) to {}, expected {}, got {}",
                ptr,
                ::std::mem::size_of::<T>(),
                align,
                expected,
                got1
            );
            ret |= true;
        }
        if got2 != expected {
            eprintln!(
                "new: aligning {:p} (with stride of {}) to {}, expected {}, got {}",
                ptr,
                ::std::mem::size_of::<T>(),
                align,
                expected,
                got2
            );
            ret |= true;
        }
        return ret;
    }

    // For pointers of stride != 1, we verify the algorithm against the naivest possible
    // implementation
    let mut align = 1;
    let mut x = false;
    while align < 1024 {
        for ptr in 1usize..4 * align {
            unsafe {
                x |= test_weird_stride::<A3>(ptr as *const A3, align);
                x |= test_weird_stride::<A4>(ptr as *const A4, align);
                x |= test_weird_stride::<A5>(ptr as *const A5, align);
                x |= test_weird_stride::<A6>(ptr as *const A6, align);
                x |= test_weird_stride::<A7>(ptr as *const A7, align);
                x |= test_weird_stride::<A8>(ptr as *const A8, align);
                x |= test_weird_stride::<A9>(ptr as *const A9, align);
                x |= test_weird_stride::<A10>(ptr as *const A10, align);
            }
        }
        align = (align + 1).next_power_of_two();
    }
    assert!(!x);
}
