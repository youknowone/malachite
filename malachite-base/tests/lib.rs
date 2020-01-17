extern crate malachite_base;

pub mod bools;
pub mod chars;
pub mod limbs;
pub mod num {
    pub mod arithmetic {
        pub mod log_two;
    }
    pub mod basic {
        pub mod constants;
        pub mod crement;
    }
    pub mod comparison;
    pub mod conversion;
    pub mod logic {
        pub mod bit_access;
        pub mod get_highest_bit;
        pub mod not_assign;
        pub mod significant_bits;
    }
}
pub mod round;
pub mod strings;
