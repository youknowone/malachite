use common::test_properties;
use malachite_base::misc::{Max, Walkable};
use malachite_base::num::{PrimitiveInteger, PrimitiveSigned, PrimitiveUnsigned};
use malachite_test::inputs::base::{signeds_no_max, unsigneds_no_max};

fn increment_helper_unsigned<T: PrimitiveInteger>() {
    let test = |mut n: u64, out| {
        n.increment();
        assert_eq!(T::from_u64(n), T::from_u64(out));
    };

    test(0, 1);
    test(1, 2);
    test(100, 101);
}

fn increment_helper_signed<T: PrimitiveSigned>() {
    increment_helper_unsigned::<T>();

    let test = |mut n: i64, out| {
        n.increment();
        assert_eq!(T::from_i64(n), T::from_i64(out));
    };

    test(-1, 0);
    test(-2, -1);
    test(-100, -99);
}

#[test]
pub fn test_increment() {
    increment_helper_unsigned::<u8>();
    increment_helper_unsigned::<u16>();
    increment_helper_unsigned::<u32>();
    increment_helper_unsigned::<u64>();
    increment_helper_signed::<i8>();
    increment_helper_signed::<i16>();
    increment_helper_signed::<i32>();
    increment_helper_signed::<i64>();
}

macro_rules! increment_fail {
    ($t: ident, $increment_fail: ident) => {
        #[test]
        #[should_panic(expected = "Cannot increment past the maximum value.")]
        fn $increment_fail() {
            let mut n = $t::MAX;
            n.increment();
        }
    };
}

increment_fail!(u8, increment_u8_fail);
increment_fail!(u16, increment_u16_fail);
increment_fail!(u32, increment_u32_fail);
increment_fail!(u64, increment_u64_fail);
increment_fail!(i8, increment_i8_fail);
increment_fail!(i16, increment_i16_fail);
increment_fail!(i32, increment_i32_fail);
increment_fail!(i64, increment_i64_fail);

fn increment_properties_helper_unsigned<T: 'static + PrimitiveUnsigned>() {
    test_properties(unsigneds_no_max, |&n: &T| {
        let mut n_mut = n;
        n_mut.increment();
        assert_ne!(n_mut, n);
        n_mut.decrement();
        assert_eq!(n_mut, n);
    });
}

fn increment_properties_helper_signed<T: 'static + PrimitiveSigned>() {
    test_properties(signeds_no_max, |&n: &T| {
        let mut n_mut = n;
        n_mut.increment();
        assert_ne!(n_mut, n);
        n_mut.decrement();
        assert_eq!(n_mut, n);
    });
}

#[test]
fn increment_properties() {
    increment_properties_helper_unsigned::<u8>();
    increment_properties_helper_unsigned::<u16>();
    increment_properties_helper_unsigned::<u32>();
    increment_properties_helper_unsigned::<u64>();
    increment_properties_helper_signed::<i8>();
    increment_properties_helper_signed::<i16>();
    increment_properties_helper_signed::<i32>();
    increment_properties_helper_signed::<i64>();
}
