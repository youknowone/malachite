use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::{
    natural_gen, natural_natural_primitive_float_triple_gen, natural_primitive_float_pair_gen,
    natural_primitive_float_primitive_float_triple_gen,
};
use std::cmp::Ordering;
use std::str::FromStr;

#[test]
fn test_partial_cmp_abs_primitive_float() {
    let test = |u, v: f32, out: Option<Ordering>| {
        let out_rev = out.map(Ordering::reverse);
        assert_eq!(Natural::from_str(u).unwrap().partial_cmp_abs(&v), out);
        assert_eq!(v.partial_cmp_abs(&Natural::from_str(u).unwrap()), out_rev);

        let v = f64::from(v);
        assert_eq!(Natural::from_str(u).unwrap().partial_cmp_abs(&v), out);
        assert_eq!(v.partial_cmp_abs(&Natural::from_str(u).unwrap()), out_rev);
    };
    test("5", f32::NAN, None);
    test("5", f32::POSITIVE_INFINITY, Some(Ordering::Less));
    test("5", f32::NEGATIVE_INFINITY, Some(Ordering::Less));

    test("0", 0.0, Some(Ordering::Equal));
    test("0", -0.0, Some(Ordering::Equal));
    test("0", 5.0, Some(Ordering::Less));
    test("0", -5.0, Some(Ordering::Less));
    test("123", 123.0, Some(Ordering::Equal));
    test("123", 5.0, Some(Ordering::Greater));
    test("123", -123.0, Some(Ordering::Equal));
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
        Some(Ordering::Less),
    );
    test(
        "1208925819614629174706176",
        -1.2089258e24,
        Some(Ordering::Equal),
    );
    test(
        "1208925819614629174706177",
        -1.2089258e24,
        Some(Ordering::Greater),
    );
}

fn partial_cmp_abs_primitive_float_properties_helper<T: PartialOrdAbs<Natural> + PrimitiveFloat>()
where
    Natural: TryFrom<T> + PartialOrd<T> + PartialOrdAbs<T>,
{
    natural_primitive_float_pair_gen::<T>().test_properties(|(n, u)| {
        let cmp_abs = n.partial_cmp_abs(&u);
        let cmp_abs_rev = cmp_abs.map(Ordering::reverse);
        assert_eq!(u.partial_cmp_abs(&n), cmp_abs_rev);

        assert_eq!(n.partial_cmp(&u.abs()), cmp_abs);
    });

    natural_natural_primitive_float_triple_gen::<T>().test_properties(|(n, m, u)| {
        if n.lt_abs(&u) && u.lt_abs(&m) {
            assert_eq!(n.cmp(&m), Ordering::Less);
        } else if n.gt_abs(&u) && u.gt_abs(&m) {
            assert_eq!(n.cmp(&m), Ordering::Greater);
        }
    });

    natural_primitive_float_primitive_float_triple_gen::<T>().test_properties(|(n, u, v)| {
        if u.lt_abs(&n) && n.lt_abs(&v) {
            assert!(u.abs() < v.abs());
        } else if u.gt_abs(&n) && n.gt_abs(&v) {
            assert!(u.abs() > v.abs());
        }
    });

    natural_gen().test_properties(|x| {
        assert!(x.ge_abs(&T::ZERO));
        assert!(x.lt_abs(&T::NEGATIVE_INFINITY));
        assert!(x.lt_abs(&T::POSITIVE_INFINITY));
    });
}

#[test]
fn partial_cmp_abs_primitive_float_properties() {
    apply_fn_to_primitive_floats!(partial_cmp_abs_primitive_float_properties_helper);
}
