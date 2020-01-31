#![feature(core_intrinsics)]

mod v0;
mod v1;
mod v2;
mod v3;
mod v4;

pub use v0::align_offset as align_offset_v0;
pub use v1::align_offset as align_offset_v1;
pub use v2::align_offset as align_offset_v2;
pub use v3::align_offset as align_offset_v3;
pub use v4::align_offset as align_offset_v4;

pub const ALIGN_OFFSET_FNS: [unsafe fn(usize, usize, usize) -> usize; 5] = [
    align_offset_v0,
    align_offset_v1,
    align_offset_v2,
    align_offset_v3,
    align_offset_v4,
];

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
        let mut ret = false;
        for i in 0..ALIGN_OFFSET_FNS.len() {
            let got = ALIGN_OFFSET_FNS[i](ptr, stride, align);
            if got != expected {
                eprintln!(
                    "align_offset_v{}: aligning {:x} (with stride of {}) to {}, expected {}, got {}",
                    i, ptr, stride, align, expected, got
                );
                ret |= true;
            }
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
