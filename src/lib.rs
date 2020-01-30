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
    unsafe fn test_weird_stride(ptr: usize, stride: usize, align: usize) -> bool {
        let mut expected = usize::max_value();
        // Naive but definitely correct way to find the *first* aligned element of stride::<T>.
        for el in 0..align {
            if (ptr + el * stride) % align == 0 {
                expected = el;
                break;
            }
        }
        let got1 = align_offset_old(ptr, stride, align);
        let got2 = align_offset_new(ptr, stride, align);
        let mut ret = false;

        if got1 != expected {
            eprintln!(
                "old: aligning {:x} (with stride of {}) to {}, expected {}, got {}",
                ptr, stride, align, expected, got1
            );
            ret |= true;
        }
        if got2 != expected {
            eprintln!(
                "new: aligning {:x} (with stride of {}) to {}, expected {}, got {}",
                ptr, stride, align, expected, got2
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
            for stride in 3..11 {
                unsafe {
                    x |= test_weird_stride(ptr, stride, align);
                }
            }
        }
        align = (align + 1).next_power_of_two();
    }
    assert!(!x);
}
