use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::{
    natural_gen, natural_natural_primitive_float_triple_gen, natural_primitive_float_pair_gen,
    natural_primitive_float_primitive_float_triple_gen,
};
use rug;
use std::cmp::Ordering;
use std::str::FromStr;

#[test]
fn test_partial_cmp_primitive_float() {
    let test = |u, v: f32, out: Option<Ordering>| {
        let out_rev = out.map(Ordering::reverse);
        assert_eq!(Natural::from_str(u).unwrap().partial_cmp(&v), out);
        assert_eq!(rug::Integer::from_str(u).unwrap().partial_cmp(&v), out);

        assert_eq!(v.partial_cmp(&Natural::from_str(u).unwrap()), out_rev);
        assert_eq!(v.partial_cmp(&rug::Integer::from_str(u).unwrap()), out_rev);

        let v = f64::from(v);
        assert_eq!(Natural::from_str(u).unwrap().partial_cmp(&v), out);
        assert_eq!(rug::Integer::from_str(u).unwrap().partial_cmp(&v), out);

        assert_eq!(v.partial_cmp(&Natural::from_str(u).unwrap()), out_rev);
        assert_eq!(v.partial_cmp(&rug::Integer::from_str(u).unwrap()), out_rev);
    };
    test("5", f32::NAN, None);
    test("5", f32::POSITIVE_INFINITY, Some(Ordering::Less));
    test("5", f32::NEGATIVE_INFINITY, Some(Ordering::Greater));

    test("0", 0.0, Some(Ordering::Equal));
    test("0", -0.0, Some(Ordering::Equal));
    test("0", 5.0, Some(Ordering::Less));
    test("0", -5.0, Some(Ordering::Greater));
    test("123", 123.0, Some(Ordering::Equal));
    test("123", 5.0, Some(Ordering::Greater));
    test("123", -123.0, Some(Ordering::Greater));
    test("1000000000000", 123.0, Some(Ordering::Greater));

    test(
        "1208925819614629174706175",
        1.2089258e24,
        Some(Ordering::Less),
    );
    test(
        "1208925819614629174706176",
        1.2089258e24,
        Some(Ordering::Equal),
    );
    test(
        "1208925819614629174706177",
        1.2089258e24,
        Some(Ordering::Greater),
    );
    test(
        "1208925819614629174706175",
        -1.2089258e24,
        Some(Ordering::Greater),
    );
    test(
        "1208925819614629174706176",
        -1.2089258e24,
        Some(Ordering::Greater),
    );
    test(
        "1208925819614629174706177",
        -1.2089258e24,
        Some(Ordering::Greater),
    );
}

#[allow(clippy::trait_duplication_in_bounds)]
fn partial_cmp_primitive_float_properties_helper<
    T: PartialOrd<Natural> + PartialOrd<rug::Integer> + PrimitiveFloat,
>()
where
    Natural: TryFrom<T> + PartialOrd<T>,
    rug::Integer: PartialOrd<T>,
{
    natural_primitive_float_pair_gen::<T>().test_properties(|(n, u)| {
        let cmp = n.partial_cmp(&u);
        assert_eq!(rug::Integer::from(&n).partial_cmp(&u), cmp);

        let cmp_rev = cmp.map(Ordering::reverse);
        assert_eq!(u.partial_cmp(&n), cmp_rev);
        assert_eq!(u.partial_cmp(&rug::Integer::from(&n)), cmp_rev);
    });

    natural_natural_primitive_float_triple_gen::<T>().test_properties(|(n, m, u)| {
        if n < u && u < m {
            assert_eq!(n.cmp(&m), Ordering::Less);
        } else if n > u && u > m {
            assert_eq!(n.cmp(&m), Ordering::Greater);
        }
    });

    natural_primitive_float_primitive_float_triple_gen::<T>().test_properties(|(n, u, v)| {
        if u < n && n < v {
            assert!(u < v);
        } else if u > n && n > v {
            assert!(u > v);
        }
    });

    natural_gen().test_properties(|x| {
        assert!(x >= T::ZERO);
        assert!(x > T::NEGATIVE_INFINITY);
        assert!(x < T::POSITIVE_INFINITY);
    });
}

#[test]
fn partial_cmp_primitive_float_properties() {
    apply_fn_to_primitive_floats!(partial_cmp_primitive_float_properties_helper);
}
