use crate::memory::address::*;
use bit_field::BitField;
use bitflags::*;

bitflags! {
    #[derive(Default)]
    pub struct Flags: u8 {
        const VALID =       1 << 0;
        const READABLE =    1 << 1;
        const WRITABLE =    1 << 2;
        const EXECUTABLE =  1 << 3;
        const USER =        1 << 4;
        const GLOBAL =      1 << 5;
        const ACCESSED =    1 << 6;
        const DIRTY =       1 << 7;
    }
}
macro_rules! implement_flags {
    ($field: ident, $name: ident, $quote: literal) => {
        impl Flags {
            #[doc = "返回 `Flags::"]
            #[doc = $quote]
            #[doc = "` 或 `Flags::empty()`"]
            pub fn $name(value: bool) -> Flags {
                if value {
                    Flags::$field
                } else {
                    Flags::empty()
                }
            }
        }
    };
}

implement_flags! {USER, user, "USER"}
implement_flags! {READABLE, readable, "READABLE"}
implement_flags! {WRITABLE, writable, "WRITABLE"}
implement_flags! {EXECUTABLE, executable, "EXECUTABLE"}

#[derive(Copy, Clone, Default)]
pub struct PageTableEntry(usize);

impl PageTableEntry {
    pub fn new(page_number: PhysicalPageNumber, flags: Flags) -> Self {
        Self(
            *0usize.set_bits(..8, flags.bits() as usize)
                .set_bits(10..54, page_number.into()),
        )
    }

    pub fn page_number(&self) -> PhysicalPageNumber {
        PhysicalPageNumber::from(self.0.get_bits(10..54))
    }

    pub fn address(&self) -> PhysicalAddress {
        PhysicalAddress::from(self.page_number())
    }

    pub fn flags(&self) -> Flags {
        unsafe {Flags::from_bits_unchecked(self.0.get_bits(..8) as u8)}
    }

    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }
}

impl core::fmt::Debug for PageTableEntry {
    fn fmt(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        formatter
            .debug_struct("PageTableEntry")
            .field("value", &self.0)
            .field("page_number", &self.page_number())
            .field("flags", &self.flags())
            .finish()
    }
}