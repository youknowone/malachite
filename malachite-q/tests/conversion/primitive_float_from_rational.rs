use malachite_base::num::arithmetic::traits::Parity;
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::{ConvertibleFrom, ExactFrom, RoundingFrom};
use malachite_base::num::float::NiceFloat;
use malachite_base::rounding_modes::RoundingMode;
use malachite_nz::integer::Integer;
use malachite_nz::natural::conversion::from_primitive_float::NaturalFromPrimitiveFloatError;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::{
    integer_gen, integer_gen_var_1, integer_rounding_mode_pair_gen_var_1,
};
use malachite_q::conversion::primitive_float_from_rational::PrimitiveFloatFromRationalError;
use malachite_q::test_util::generators::{
    rational_gen, rational_gen_var_4, rational_gen_var_5, rational_gen_var_6,
    rational_rounding_mode_pair_gen_var_5,
};
use malachite_q::Rational;
use std::panic::catch_unwind;
use std::str::FromStr;

#[test]
fn test_f32_rounding_from_rational() {
    let max = Rational::exact_from(f32::MAX_FINITE);
    let test = |s: &str, rm: RoundingMode, out| {
        let u = Rational::from_str(s).unwrap();
        assert_eq!(NiceFloat(f32::rounding_from(&u, rm)), NiceFloat(out));
        assert_eq!(NiceFloat(f32::rounding_from(u.clone(), rm)), NiceFloat(out));
        if rm == RoundingMode::Down && u.lt_abs(&max) && NiceFloat(out) != NiceFloat(-0.0) {
            assert_eq!(
                NiceFloat(rug::Rational::from_str(s).unwrap().to_f32()),
                NiceFloat(out)
            );
        }
    };
    test("3", RoundingMode::Exact, 3.0);
    test("-3", RoundingMode::Exact, -3.0);
    test("123", RoundingMode::Exact, 123.0);
    test("-123", RoundingMode::Exact, -123.0);
    test("0", RoundingMode::Exact, 0.0);
    test("1000000000", RoundingMode::Exact, 1.0e9);
    test("-1000000000", RoundingMode::Exact, -1.0e9);
    test("16777216", RoundingMode::Exact, 1.6777216e7);
    test("-16777216", RoundingMode::Exact, -1.6777216e7);
    test("16777218", RoundingMode::Exact, 1.6777218e7);
    test("-16777218", RoundingMode::Exact, -1.6777218e7);
    test("16777217", RoundingMode::Floor, 1.6777216e7);
    test("16777217", RoundingMode::Down, 1.6777216e7);
    test("16777217", RoundingMode::Ceiling, 1.6777218e7);
    test("16777217", RoundingMode::Up, 1.6777218e7);
    test("16777217", RoundingMode::Nearest, 1.6777216e7);
    test("-16777217", RoundingMode::Floor, -1.6777218e7);
    test("-16777217", RoundingMode::Down, -1.6777216e7);
    test("-16777217", RoundingMode::Ceiling, -1.6777216e7);
    test("-16777217", RoundingMode::Up, -1.6777218e7);
    test("-16777217", RoundingMode::Nearest, -1.6777216e7);
    test("33554432", RoundingMode::Exact, 3.3554432e7);
    test("-33554432", RoundingMode::Exact, -3.3554432e7);
    test("33554436", RoundingMode::Exact, 3.3554436e7);
    test("-33554436", RoundingMode::Exact, -3.3554436e7);
    test("33554433", RoundingMode::Floor, 3.3554432e7);
    test("33554433", RoundingMode::Down, 3.3554432e7);
    test("33554433", RoundingMode::Ceiling, 3.3554436e7);
    test("33554433", RoundingMode::Up, 3.3554436e7);
    test("33554433", RoundingMode::Nearest, 3.3554432e7);
    test("-33554433", RoundingMode::Floor, -3.3554436e7);
    test("-33554433", RoundingMode::Down, -3.3554432e7);
    test("-33554433", RoundingMode::Ceiling, -3.3554432e7);
    test("-33554433", RoundingMode::Up, -3.3554436e7);
    test("-33554433", RoundingMode::Nearest, -3.3554432e7);
    test("33554434", RoundingMode::Nearest, 3.3554432e7);
    test("-33554434", RoundingMode::Nearest, -3.3554432e7);
    test("33554435", RoundingMode::Nearest, 3.3554436e7);
    test("-33554435", RoundingMode::Nearest, -3.3554436e7);
    test(
        "340282346638528859811704183484516925439",
        RoundingMode::Floor,
        3.4028233e38,
    );
    test(
        "340282346638528859811704183484516925439",
        RoundingMode::Down,
        3.4028233e38,
    );
    test(
        "340282346638528859811704183484516925439",
        RoundingMode::Ceiling,
        3.4028235e38,
    );
    test(
        "340282346638528859811704183484516925439",
        RoundingMode::Up,
        3.4028235e38,
    );
    test(
        "340282346638528859811704183484516925439",
        RoundingMode::Nearest,
        3.4028235e38,
    );
    test(
        "-340282346638528859811704183484516925439",
        RoundingMode::Floor,
        -3.4028235e38,
    );
    test(
        "-340282346638528859811704183484516925439",
        RoundingMode::Down,
        -3.4028233e38,
    );
    test(
        "-340282346638528859811704183484516925439",
        RoundingMode::Ceiling,
        -3.4028233e38,
    );
    test(
        "-340282346638528859811704183484516925439",
        RoundingMode::Up,
        -3.4028235e38,
    );
    test(
        "-340282346638528859811704183484516925439",
        RoundingMode::Nearest,
        -3.4028235e38,
    );
    test(
        "340282346638528859811704183484516925440",
        RoundingMode::Exact,
        3.4028235e38,
    );
    test(
        "-340282346638528859811704183484516925440",
        RoundingMode::Exact,
        -3.4028235e38,
    );
    test(
        "340282346638528859811704183484516925441",
        RoundingMode::Floor,
        3.4028235e38,
    );
    test(
        "340282346638528859811704183484516925441",
        RoundingMode::Down,
        3.4028235e38,
    );
    test(
        "340282346638528859811704183484516925441",
        RoundingMode::Nearest,
        3.4028235e38,
    );
    test(
        "340282346638528859811704183484516925441",
        RoundingMode::Ceiling,
        f32::POSITIVE_INFINITY,
    );
    test(
        "340282346638528859811704183484516925441",
        RoundingMode::Up,
        f32::POSITIVE_INFINITY,
    );
    test(
        "-340282346638528859811704183484516925441",
        RoundingMode::Floor,
        f32::NEGATIVE_INFINITY,
    );
    test(
        "-340282346638528859811704183484516925441",
        RoundingMode::Down,
        -3.4028235e38,
    );
    test(
        "-340282346638528859811704183484516925441",
        RoundingMode::Nearest,
        -3.4028235e38,
    );
    test(
        "-340282346638528859811704183484516925441",
        RoundingMode::Ceiling,
        -3.4028235e38,
    );
    test(
        "-340282346638528859811704183484516925441",
        RoundingMode::Up,
        f32::NEGATIVE_INFINITY,
    );
    test(
        "10000000000000000000000000000000000000000000000000000",
        RoundingMode::Floor,
        3.4028235e38,
    );
    test(
        "10000000000000000000000000000000000000000000000000000",
        RoundingMode::Down,
        3.4028235e38,
    );
    test(
        "10000000000000000000000000000000000000000000000000000",
        RoundingMode::Nearest,
        3.4028235e38,
    );
    test(
        "10000000000000000000000000000000000000000000000000000",
        RoundingMode::Ceiling,
        f32::POSITIVE_INFINITY,
    );
    test(
        "10000000000000000000000000000000000000000000000000000",
        RoundingMode::Up,
        f32::POSITIVE_INFINITY,
    );
    test(
        "-10000000000000000000000000000000000000000000000000000",
        RoundingMode::Floor,
        f32::NEGATIVE_INFINITY,
    );
    test(
        "-10000000000000000000000000000000000000000000000000000",
        RoundingMode::Down,
        -3.4028235e38,
    );
    test(
        "-10000000000000000000000000000000000000000000000000000",
        RoundingMode::Nearest,
        -3.4028235e38,
    );
    test(
        "-10000000000000000000000000000000000000000000000000000",
        RoundingMode::Ceiling,
        -3.4028235e38,
    );
    test(
        "-10000000000000000000000000000000000000000000000000000",
        RoundingMode::Up,
        f32::NEGATIVE_INFINITY,
    );
    test("1125899873419263", RoundingMode::Floor, 1.12589984e15);
    test("1125899873419263", RoundingMode::Down, 1.12589984e15);
    test("1125899873419263", RoundingMode::Ceiling, 1.1258999e15);
    test("1125899873419263", RoundingMode::Up, 1.1258999e15);
    test("1125899873419263", RoundingMode::Nearest, 1.1258999e15);
    test("-1125899873419263", RoundingMode::Floor, -1.1258999e15);
    test("-1125899873419263", RoundingMode::Down, -1.12589984e15);
    test("-1125899873419263", RoundingMode::Ceiling, -1.12589984e15);
    test("-1125899873419263", RoundingMode::Up, -1.1258999e15);
    test("-1125899873419263", RoundingMode::Nearest, -1.1258999e15);

    test("1/2", RoundingMode::Floor, 0.5);
    test("1/2", RoundingMode::Down, 0.5);
    test("1/2", RoundingMode::Ceiling, 0.5);
    test("1/2", RoundingMode::Up, 0.5);
    test("1/2", RoundingMode::Nearest, 0.5);
    test("1/2", RoundingMode::Exact, 0.5);

    test("-1/2", RoundingMode::Floor, -0.5);
    test("-1/2", RoundingMode::Down, -0.5);
    test("-1/2", RoundingMode::Ceiling, -0.5);
    test("-1/2", RoundingMode::Up, -0.5);
    test("-1/2", RoundingMode::Nearest, -0.5);
    test("-1/2", RoundingMode::Exact, -0.5);

    test("1/3", RoundingMode::Floor, 0.3333333);
    test("1/3", RoundingMode::Down, 0.3333333);
    test("1/3", RoundingMode::Ceiling, 0.33333334);
    test("1/3", RoundingMode::Up, 0.33333334);
    test("1/3", RoundingMode::Nearest, 0.33333334);

    test("-1/3", RoundingMode::Floor, -0.33333334);
    test("-1/3", RoundingMode::Down, -0.3333333);
    test("-1/3", RoundingMode::Ceiling, -0.3333333);
    test("-1/3", RoundingMode::Up, -0.33333334);
    test("-1/3", RoundingMode::Nearest, -0.33333334);

    // subnormal
    test(
        "1/10000000000000000000000000000000000000000",
        RoundingMode::Floor,
        1.0e-40,
    );
    test(
        "1/10000000000000000000000000000000000000000",
        RoundingMode::Down,
        1.0e-40,
    );
    test(
        "1/10000000000000000000000000000000000000000",
        RoundingMode::Ceiling,
        1.00001e-40,
    );
    test(
        "1/10000000000000000000000000000000000000000",
        RoundingMode::Up,
        1.00001e-40,
    );
    test(
        "1/10000000000000000000000000000000000000000",
        RoundingMode::Nearest,
        1.0e-40,
    );

    test(
        "-1/10000000000000000000000000000000000000000",
        RoundingMode::Floor,
        -1.00001e-40,
    );
    test(
        "-1/10000000000000000000000000000000000000000",
        RoundingMode::Down,
        -1.0e-40,
    );
    test(
        "-1/10000000000000000000000000000000000000000",
        RoundingMode::Ceiling,
        -1.0e-40,
    );
    test(
        "-1/10000000000000000000000000000000000000000",
        RoundingMode::Up,
        -1.00001e-40,
    );
    test(
        "-1/10000000000000000000000000000000000000000",
        RoundingMode::Nearest,
        -1.0e-40,
    );

    // less than subnormal
    test(
        "1/100000000000000000000000000000000000000000000000000",
        RoundingMode::Floor,
        0.0,
    );
    test(
        "1/100000000000000000000000000000000000000000000000000",
        RoundingMode::Down,
        0.0,
    );
    test(
        "1/100000000000000000000000000000000000000000000000000",
        RoundingMode::Ceiling,
        1.0e-45,
    );
    test(
        "1/100000000000000000000000000000000000000000000000000",
        RoundingMode::Up,
        1.0e-45,
    );
    test(
        "1/100000000000000000000000000000000000000000000000000",
        RoundingMode::Nearest,
        0.0,
    );

    test(
        "-1/100000000000000000000000000000000000000000000000000",
        RoundingMode::Floor,
        -1.0e-45,
    );
    test(
        "-1/100000000000000000000000000000000000000000000000000",
        RoundingMode::Down,
        -0.0,
    );
    test(
        "-1/100000000000000000000000000000000000000000000000000",
        RoundingMode::Ceiling,
        -0.0,
    );
    test(
        "-1/100000000000000000000000000000000000000000000000000",
        RoundingMode::Up,
        -1.0e-45,
    );
    test(
        "-1/100000000000000000000000000000000000000000000000000",
        RoundingMode::Nearest,
        -0.0,
    );

    // half of smallest positive
    test(
        "1/1427247692705959881058285969449495136382746624",
        RoundingMode::Floor,
        0.0,
    );
    test(
        "1/1427247692705959881058285969449495136382746624",
        RoundingMode::Down,
        0.0,
    );
    test(
        "1/1427247692705959881058285969449495136382746624",
        RoundingMode::Ceiling,
        1.0e-45,
    );
    test(
        "1/1427247692705959881058285969449495136382746624",
        RoundingMode::Up,
        1.0e-45,
    );
    test(
        "1/1427247692705959881058285969449495136382746624",
        RoundingMode::Nearest,
        0.0,
    );

    test(
        "-1/1427247692705959881058285969449495136382746624",
        RoundingMode::Floor,
        -1.0e-45,
    );
    test(
        "-1/1427247692705959881058285969449495136382746624",
        RoundingMode::Down,
        -0.0,
    );
    test(
        "-1/1427247692705959881058285969449495136382746624",
        RoundingMode::Ceiling,
        -0.0,
    );
    test(
        "-1/1427247692705959881058285969449495136382746624",
        RoundingMode::Up,
        -1.0e-45,
    );
    test(
        "-1/1427247692705959881058285969449495136382746624",
        RoundingMode::Nearest,
        -0.0,
    );

    // just over half of smallest positive; Nearest rounds up
    test(
        "88819109620612751463292030150471001/126765060022822940149670320537600000000000000000000000\
        000000000000000000000000000",
        RoundingMode::Floor,
        0.0
    );
    test(
        "88819109620612751463292030150471001/126765060022822940149670320537600000000000000000000000\
        000000000000000000000000000",
        RoundingMode::Down,
        0.0
    );
    test(
        "88819109620612751463292030150471001/126765060022822940149670320537600000000000000000000000\
        000000000000000000000000000",
        RoundingMode::Ceiling,
        1.0e-45
    );
    test(
        "88819109620612751463292030150471001/126765060022822940149670320537600000000000000000000000\
        000000000000000000000000000",
        RoundingMode::Up,
        1.0e-45
    );
    test(
        "88819109620612751463292030150471001/126765060022822940149670320537600000000000000000000000\
        000000000000000000000000000",
        RoundingMode::Nearest,
        1.0e-45
    );

    test(
        "-88819109620612751463292030150471001/12676506002282294014967032053760000000000000000000000\
        0000000000000000000000000000",
        RoundingMode::Floor,
        -1.0e-45
    );
    test(
        "-88819109620612751463292030150471001/12676506002282294014967032053760000000000000000000000\
        0000000000000000000000000000",
        RoundingMode::Down,
        -0.0
    );
    test(
        "-88819109620612751463292030150471001/12676506002282294014967032053760000000000000000000000\
        0000000000000000000000000000",
        RoundingMode::Ceiling,
        -0.0
    );
    test(
        "-88819109620612751463292030150471001/12676506002282294014967032053760000000000000000000000\
        0000000000000000000000000000",
        RoundingMode::Up,
        -1.0e-45
    );
    test(
        "-88819109620612751463292030150471001/12676506002282294014967032053760000000000000000000000\
        0000000000000000000000000000",
        RoundingMode::Nearest,
        -1.0e-45
    );

    // halfway between max subnormal and min normal
    test(
        "16777215/1427247692705959881058285969449495136382746624",
        RoundingMode::Floor,
        1.1754942e-38,
    );
    test(
        "16777215/1427247692705959881058285969449495136382746624",
        RoundingMode::Down,
        1.1754942e-38,
    );
    test(
        "16777215/1427247692705959881058285969449495136382746624",
        RoundingMode::Ceiling,
        1.1754944e-38,
    );
    test(
        "16777215/1427247692705959881058285969449495136382746624",
        RoundingMode::Up,
        1.1754944e-38,
    );
    test(
        "16777215/1427247692705959881058285969449495136382746624",
        RoundingMode::Nearest,
        1.1754944e-38,
    );
}

#[test]
fn f32_rounding_from_rational_fail() {
    assert_panic!(f32::rounding_from(
        Rational::from_str("340282346638528859811704183484516925439").unwrap(),
        RoundingMode::Exact,
    ));
    assert_panic!(f32::rounding_from(
        Rational::from_str("340282346638528859811704183484516925441").unwrap(),
        RoundingMode::Exact,
    ));
    assert_panic!(f32::rounding_from(
        Rational::from_str("16777217").unwrap(),
        RoundingMode::Exact
    ));
    assert_panic!(f32::rounding_from(
        Rational::from_str("10000000000000000000000000000000000000000000000000000").unwrap(),
        RoundingMode::Exact,
    ));
    assert_panic!(f32::rounding_from(
        Rational::from_str("1/10").unwrap(),
        RoundingMode::Exact
    ));
}

#[test]
fn f32_rounding_from_rational_ref_fail() {
    assert_panic!(f32::rounding_from(
        &Rational::from_str("340282346638528859811704183484516925439").unwrap(),
        RoundingMode::Exact,
    ));
    assert_panic!(f32::rounding_from(
        &Rational::from_str("340282346638528859811704183484516925441").unwrap(),
        RoundingMode::Exact,
    ));
    assert_panic!(f32::rounding_from(
        &Rational::from_str("16777217").unwrap(),
        RoundingMode::Exact,
    ));
    assert_panic!(f32::rounding_from(
        &Rational::from_str("10000000000000000000000000000000000000000000000000000").unwrap(),
        RoundingMode::Exact,
    ));
    assert_panic!(f32::rounding_from(
        &Rational::from_str("1/10").unwrap(),
        RoundingMode::Exact
    ));
}

#[test]
fn test_f64_rounding_from_rational() {
    let test = |s: &str, rm: RoundingMode, out| {
        let u = Rational::from_str(s).unwrap();
        assert_eq!(NiceFloat(f64::rounding_from(&u, rm)), NiceFloat(out));
        assert_eq!(NiceFloat(f64::rounding_from(u, rm)), NiceFloat(out));
        if rm == RoundingMode::Down {
            assert_eq!(
                NiceFloat(rug::Rational::from_str(s).unwrap().to_f64()),
                NiceFloat(out)
            );
        }
    };
    test("3", RoundingMode::Exact, 3.0);
    test("-3", RoundingMode::Exact, -3.0);
    test("123", RoundingMode::Exact, 123.0);
    test("-123", RoundingMode::Exact, -123.0);
    test("0", RoundingMode::Exact, 0.0);
    test("100000000000000000000", RoundingMode::Exact, 1.0e20);
    test("-100000000000000000000", RoundingMode::Exact, -1.0e20);
    test(
        "9007199254740992",
        RoundingMode::Exact,
        9.007199254740992e15,
    );
    test(
        "-9007199254740992",
        RoundingMode::Exact,
        -9.007199254740992e15,
    );
    test(
        "9007199254740994",
        RoundingMode::Exact,
        9.007199254740994e15,
    );
    test(
        "-9007199254740994",
        RoundingMode::Exact,
        -9.007199254740994e15,
    );
    test(
        "9007199254740993",
        RoundingMode::Floor,
        9.007199254740992e15,
    );
    test("9007199254740993", RoundingMode::Down, 9.007199254740992e15);
    test(
        "9007199254740993",
        RoundingMode::Ceiling,
        9.007199254740994e15,
    );
    test("9007199254740993", RoundingMode::Up, 9.007199254740994e15);
    test(
        "9007199254740993",
        RoundingMode::Nearest,
        9.007199254740992e15,
    );
    test(
        "-9007199254740993",
        RoundingMode::Floor,
        -9.007199254740994e15,
    );
    test(
        "-9007199254740993",
        RoundingMode::Down,
        -9.007199254740992e15,
    );
    test(
        "-9007199254740993",
        RoundingMode::Ceiling,
        -9.007199254740992e15,
    );
    test("-9007199254740993", RoundingMode::Up, -9.007199254740994e15);
    test(
        "-9007199254740993",
        RoundingMode::Nearest,
        -9.007199254740992e15,
    );
    test(
        "18014398509481984",
        RoundingMode::Exact,
        1.8014398509481984e16,
    );
    test(
        "-18014398509481984",
        RoundingMode::Exact,
        -1.8014398509481984e16,
    );
    test(
        "18014398509481988",
        RoundingMode::Exact,
        1.8014398509481988e16,
    );
    test(
        "-18014398509481988",
        RoundingMode::Exact,
        -1.8014398509481988e16,
    );
    test(
        "18014398509481985",
        RoundingMode::Floor,
        1.8014398509481984e16,
    );
    test(
        "18014398509481985",
        RoundingMode::Down,
        1.8014398509481984e16,
    );
    test(
        "18014398509481985",
        RoundingMode::Ceiling,
        1.8014398509481988e16,
    );
    test("18014398509481985", RoundingMode::Up, 1.8014398509481988e16);
    test(
        "18014398509481985",
        RoundingMode::Nearest,
        1.8014398509481984e16,
    );
    test(
        "-18014398509481985",
        RoundingMode::Floor,
        -1.8014398509481988e16,
    );
    test(
        "-18014398509481985",
        RoundingMode::Down,
        -1.8014398509481984e16,
    );
    test(
        "-18014398509481985",
        RoundingMode::Ceiling,
        -1.8014398509481984e16,
    );
    test(
        "-18014398509481985",
        RoundingMode::Up,
        -1.8014398509481988e16,
    );
    test(
        "-18014398509481985",
        RoundingMode::Nearest,
        -1.8014398509481984e16,
    );
    test(
        "18014398509481986",
        RoundingMode::Nearest,
        1.8014398509481984e16,
    );
    test(
        "-18014398509481986",
        RoundingMode::Nearest,
        -1.8014398509481984e16,
    );
    test(
        "18014398509481987",
        RoundingMode::Nearest,
        1.8014398509481988e16,
    );
    test(
        "-18014398509481987",
        RoundingMode::Nearest,
        -1.8014398509481988e16,
    );

    test("1/2", RoundingMode::Floor, 0.5);
    test("1/2", RoundingMode::Down, 0.5);
    test("1/2", RoundingMode::Ceiling, 0.5);
    test("1/2", RoundingMode::Up, 0.5);
    test("1/2", RoundingMode::Nearest, 0.5);
    test("1/2", RoundingMode::Exact, 0.5);

    test("-1/2", RoundingMode::Floor, -0.5);
    test("-1/2", RoundingMode::Down, -0.5);
    test("-1/2", RoundingMode::Ceiling, -0.5);
    test("-1/2", RoundingMode::Up, -0.5);
    test("-1/2", RoundingMode::Nearest, -0.5);
    test("-1/2", RoundingMode::Exact, -0.5);

    test("1/3", RoundingMode::Floor, 0.3333333333333333);
    test("1/3", RoundingMode::Down, 0.3333333333333333);
    test("1/3", RoundingMode::Ceiling, 0.33333333333333337);
    test("1/3", RoundingMode::Up, 0.33333333333333337);
    test("1/3", RoundingMode::Nearest, 0.3333333333333333);

    test("-1/3", RoundingMode::Floor, -0.33333333333333337);
    test("-1/3", RoundingMode::Down, -0.3333333333333333);
    test("-1/3", RoundingMode::Ceiling, -0.3333333333333333);
    test("-1/3", RoundingMode::Up, -0.33333333333333337);
    test("-1/3", RoundingMode::Nearest, -0.3333333333333333);
}

#[test]
fn f64_rounding_from_rational_fail() {
    assert_panic!(f64::rounding_from(Rational::from_str(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858367").unwrap(), RoundingMode::Exact)
    );
    assert_panic!(f64::rounding_from(Rational::from_str(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858369").unwrap(), RoundingMode::Exact)
    );
    assert_panic!(f64::rounding_from(
        Rational::from_str("9007199254740993").unwrap(),
        RoundingMode::Exact,
    ));
    assert_panic!(f64::rounding_from(
        Rational::from_str("1/10").unwrap(),
        RoundingMode::Exact
    ));
}

#[test]
fn f64_rounding_from_rational_ref_fail() {
    assert_panic!(
        f64::rounding_from(&Rational::from_str(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858367").unwrap(), RoundingMode::Exact)
    );
    assert_panic!(
        f64::rounding_from(&Rational::from_str(
        "17976931348623157081452742373170435679807056752584499659891747680315726078002858760589558\
        632766878171540458953514382464234321326889464182768467546703537516986049910576552820762454\
        900903893289440758685084551339423045832369032229481658085593321233482747978262044472316873\
        8177180919299881250404026184124858369").unwrap(), RoundingMode::Exact)
    );
    assert_panic!(f64::rounding_from(
        &Rational::from_str("9007199254740993").unwrap(),
        RoundingMode::Exact,
    ));
    assert_panic!(f64::rounding_from(
        &Rational::from_str("1/10").unwrap(),
        RoundingMode::Exact
    ));
}

#[test]
fn test_f32_try_from_rational() {
    let test = |s: &str, out: Result<f32, PrimitiveFloatFromRationalError>| {
        let u = Rational::from_str(s).unwrap();
        assert_eq!(f32::try_from(&u).map(NiceFloat), out.map(NiceFloat));
        assert_eq!(f32::try_from(u.clone()).map(NiceFloat), out.map(NiceFloat));
        assert_eq!(f32::convertible_from(u), out.is_ok());
    };
    test("3", Ok(3.0));
    test("-3", Ok(-3.0));
    test("123", Ok(123.0));
    test("-123", Ok(-123.0));
    test("0", Ok(0.0));
    test("1000000000", Ok(1.0e9));
    test("-1000000000", Ok(-1.0e9));
    test("16777216", Ok(1.6777216e7));
    test("-16777216", Ok(-1.6777216e7));
    test("16777218", Ok(1.6777218e7));
    test("-16777218", Ok(-1.6777218e7));
    test("16777217", Err(PrimitiveFloatFromRationalError));
    test("-16777217", Err(PrimitiveFloatFromRationalError));
    test("33554432", Ok(3.3554432e7));
    test("-33554432", Ok(-3.3554432e7));
    test("33554436", Ok(3.3554436e7));
    test("-33554436", Ok(-3.3554436e7));
    test("33554433", Err(PrimitiveFloatFromRationalError));
    test("-33554433", Err(PrimitiveFloatFromRationalError));
    test("33554434", Err(PrimitiveFloatFromRationalError));
    test("-33554434", Err(PrimitiveFloatFromRationalError));
    test("33554435", Err(PrimitiveFloatFromRationalError));
    test("-33554435", Err(PrimitiveFloatFromRationalError));
    test(
        "340282346638528859811704183484516925439",
        Err(PrimitiveFloatFromRationalError),
    );
    test(
        "-340282346638528859811704183484516925439",
        Err(PrimitiveFloatFromRationalError),
    );
    test("340282346638528859811704183484516925440", Ok(3.4028235e38));
    test(
        "-340282346638528859811704183484516925440",
        Ok(-3.4028235e38),
    );
    test(
        "340282346638528859811704183484516925441",
        Err(PrimitiveFloatFromRationalError),
    );
    test(
        "-340282346638528859811704183484516925441",
        Err(PrimitiveFloatFromRationalError),
    );
    test(
        "10000000000000000000000000000000000000000000000000000",
        Err(PrimitiveFloatFromRationalError),
    );
    test(
        "-10000000000000000000000000000000000000000000000000000",
        Err(PrimitiveFloatFromRationalError),
    );

    test("1/2", Ok(0.5));
    test("-1/2", Ok(-0.5));
    test("1/3", Err(PrimitiveFloatFromRationalError));
    test("-1/3", Err(PrimitiveFloatFromRationalError));
    test(
        "1/713623846352979940529142984724747568191373312",
        Ok(f32::MIN_POSITIVE_SUBNORMAL),
    );
    test(
        "-1/713623846352979940529142984724747568191373312",
        Ok(-f32::MIN_POSITIVE_SUBNORMAL),
    );
    test(
        "8388607/713623846352979940529142984724747568191373312",
        Ok(f32::MAX_SUBNORMAL),
    );
    test(
        "-8388607/713623846352979940529142984724747568191373312",
        Ok(-f32::MAX_SUBNORMAL),
    );
    test(
        "1/85070591730234615865843651857942052864",
        Ok(f32::MIN_POSITIVE_NORMAL),
    );
    test(
        "-1/85070591730234615865843651857942052864",
        Ok(-f32::MIN_POSITIVE_NORMAL),
    );
}

#[test]
fn test_f64_try_from_rational() {
    let test = |s: &str, out: Result<f64, PrimitiveFloatFromRationalError>| {
        let u = Rational::from_str(s).unwrap();
        assert_eq!(f64::try_from(&u).map(NiceFloat), out.map(NiceFloat));
        assert_eq!(f64::try_from(u.clone()).map(NiceFloat), out.map(NiceFloat));
        assert_eq!(f64::convertible_from(u), out.is_ok());
    };
    test("3", Ok(3.0));
    test("-3", Ok(-3.0));
    test("123", Ok(123.0));
    test("-123", Ok(-123.0));
    test("0", Ok(0.0));
    test("1000000000", Ok(1.0e9));
    test("-1000000000", Ok(-1.0e9));
    test("9007199254740992", Ok(9.007199254740992e15));
    test("-9007199254740992", Ok(-9.007199254740992e15));
    test("9007199254740994", Ok(9.007199254740994e15));
    test("-9007199254740994", Ok(-9.007199254740994e15));
    test("9007199254740993", Err(PrimitiveFloatFromRationalError));
    test("-9007199254740993", Err(PrimitiveFloatFromRationalError));
    test("18014398509481984", Ok(1.8014398509481984e16));
    test("-18014398509481984", Ok(-1.8014398509481984e16));
    test("18014398509481988", Ok(1.8014398509481988e16));
    test("-18014398509481988", Ok(-1.8014398509481988e16));
    test("18014398509481985", Err(PrimitiveFloatFromRationalError));
    test("-18014398509481985", Err(PrimitiveFloatFromRationalError));
    test("18014398509481986", Err(PrimitiveFloatFromRationalError));
    test("-18014398509481986", Err(PrimitiveFloatFromRationalError));
    test("18014398509481987", Err(PrimitiveFloatFromRationalError));
    test("-18014398509481987", Err(PrimitiveFloatFromRationalError));
    test(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858367", Err(PrimitiveFloatFromRationalError));
    test(
        "-17976931348623157081452742373170435679807056752584499659891747680315726078002853876058955\
        8632766878171540458953514382464234321326889464182768467546703537516986049910576551282076245\
        4900903893289440758685084551339423045832369032229481658085593321233482747978262041447231687\
        38177180919299881250404026184124858367", Err(PrimitiveFloatFromRationalError));
    test(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858368", Ok(1.7976931348623157e308));
    test(
        "-17976931348623157081452742373170435679807056752584499659891747680315726078002853876058955\
        8632766878171540458953514382464234321326889464182768467546703537516986049910576551282076245\
        4900903893289440758685084551339423045832369032229481658085593321233482747978262041447231687\
        38177180919299881250404026184124858368", Ok(-1.7976931348623157e308));
    test(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858369", Err(PrimitiveFloatFromRationalError));
    test(
        "-17976931348623157081452742373170435679807056752584499659891747680315726078002853876058955\
        8632766878171540458953514382464234321326889464182768467546703537516986049910576551282076245\
        4900903893289440758685084551339423045832369032229481658085593321233482747978262041447231687\
        38177180919299881250404026184124858369", Err(PrimitiveFloatFromRationalError));

    test("1/2", Ok(0.5));
    test("-1/2", Ok(-0.5));
    test("1/3", Err(PrimitiveFloatFromRationalError));
    test("-1/3", Err(PrimitiveFloatFromRationalError));
}

#[test]
fn test_f32_exact_from_rational() {
    let test = |s: &str, out| {
        let u = Rational::from_str(s).unwrap();
        assert_eq!(NiceFloat(f32::exact_from(&u)), NiceFloat(out));
        assert_eq!(NiceFloat(f32::exact_from(u)), NiceFloat(out));
    };
    test("3", 3.0);
    test("-3", -3.0);
    test("123", 123.0);
    test("-123", -123.0);
    test("0", 0.0);
    test("1000000000", 1.0e9);
    test("-1000000000", -1.0e9);
    test("16777216", 1.6777216e7);
    test("-16777216", -1.6777216e7);
    test("16777218", 1.6777218e7);
    test("-16777218", -1.6777218e7);
    test("33554432", 3.3554432e7);
    test("-33554432", -3.3554432e7);
    test("33554436", 3.3554436e7);
    test("-33554436", -3.3554436e7);
    test("340282346638528859811704183484516925440", 3.4028235e38);
    test("-340282346638528859811704183484516925440", -3.4028235e38);
}

#[test]
fn f32_exact_from_rational_fail() {
    assert_panic!(f32::exact_from(Rational::from_str("16777217").unwrap()));
    assert_panic!(f32::exact_from(Rational::from_str("-16777217").unwrap()));
    assert_panic!(f32::exact_from(Rational::from_str("33554433").unwrap()));
    assert_panic!(f32::exact_from(Rational::from_str("-33554433").unwrap()));
    assert_panic!(f32::exact_from(Rational::from_str("33554434").unwrap()));
    assert_panic!(f32::exact_from(Rational::from_str("-33554434").unwrap()));
    assert_panic!(f32::exact_from(Rational::from_str("33554435").unwrap()));
    assert_panic!(f32::exact_from(Rational::from_str("-33554435").unwrap()));
    assert_panic!(f32::exact_from(
        Rational::from_str("340282346638528859811704183484516925439").unwrap()
    ));
    assert_panic!(f32::exact_from(
        Rational::from_str("-340282346638528859811704183484516925439").unwrap()
    ));
    assert_panic!(f32::exact_from(
        Rational::from_str("340282346638528859811704183484516925441").unwrap()
    ));
    assert_panic!(f32::exact_from(
        Rational::from_str("-340282346638528859811704183484516925441").unwrap()
    ));
    assert_panic!(f32::exact_from(
        Rational::from_str("340282346638528859811704183484516925441").unwrap()
    ));
    assert_panic!(f32::exact_from(
        Rational::from_str("-340282346638528859811704183484516925441").unwrap()
    ));
    assert_panic!(f32::exact_from(
        Rational::from_str("10000000000000000000000000000000000000000000000000000").unwrap(),
    ));
    assert_panic!(f32::exact_from(
        Rational::from_str("-10000000000000000000000000000000000000000000000000000").unwrap(),
    ));
    assert_panic!(f32::exact_from(Rational::from_str("1/3").unwrap()));
    assert_panic!(f32::exact_from(Rational::from_str("-1/3").unwrap()));
}

#[test]
fn f32_exact_from_rational_ref_fail() {
    assert_panic!(f32::exact_from(&Rational::from_str("16777217").unwrap()));
    assert_panic!(f32::exact_from(&Rational::from_str("-16777217").unwrap()));
    assert_panic!(f32::exact_from(&Rational::from_str("33554433").unwrap()));
    assert_panic!(f32::exact_from(&Rational::from_str("-33554433").unwrap()));
    assert_panic!(f32::exact_from(&Rational::from_str("33554434").unwrap()));
    assert_panic!(f32::exact_from(&Rational::from_str("-33554434").unwrap()));
    assert_panic!(f32::exact_from(&Rational::from_str("33554435").unwrap()));
    assert_panic!(f32::exact_from(&Rational::from_str("-33554435").unwrap()));
    assert_panic!(f32::exact_from(
        &Rational::from_str("340282346638528859811704183484516925439").unwrap()
    ));
    assert_panic!(f32::exact_from(
        &Rational::from_str("-340282346638528859811704183484516925439").unwrap()
    ));
    assert_panic!(f32::exact_from(
        &Rational::from_str("340282346638528859811704183484516925441").unwrap()
    ));
    assert_panic!(f32::exact_from(
        &Rational::from_str("-340282346638528859811704183484516925441").unwrap()
    ));
    assert_panic!(f32::exact_from(
        &Rational::from_str("340282346638528859811704183484516925441").unwrap()
    ));
    assert_panic!(f32::exact_from(
        &Rational::from_str("-340282346638528859811704183484516925441").unwrap()
    ));
    assert_panic!(f32::exact_from(
        &Rational::from_str("10000000000000000000000000000000000000000000000000000").unwrap(),
    ));
    assert_panic!(f32::exact_from(
        &Rational::from_str("-10000000000000000000000000000000000000000000000000000").unwrap(),
    ));
    assert_panic!(f32::exact_from(&Rational::from_str("1/3").unwrap()));
    assert_panic!(f32::exact_from(&Rational::from_str("-1/3").unwrap()));
}

#[test]
fn test_f64_exact_from_rational() {
    let test = |s: &str, out| {
        let u = Rational::from_str(s).unwrap();
        assert_eq!(NiceFloat(f64::exact_from(&u)), NiceFloat(out));
        assert_eq!(NiceFloat(f64::exact_from(u)), NiceFloat(out));
    };
    test("3", 3.0);
    test("-3", -3.0);
    test("123", 123.0);
    test("-123", -123.0);
    test("0", 0.0);
    test("1000000000", 1.0e9);
    test("-1000000000", -1.0e9);
    test("9007199254740992", 9.007199254740992e15);
    test("-9007199254740992", -9.007199254740992e15);
    test("9007199254740994", 9.007199254740994e15);
    test("-9007199254740994", -9.007199254740994e15);
    test("18014398509481984", 1.8014398509481984e16);
    test("-18014398509481984", -1.8014398509481984e16);
    test("18014398509481988", 1.8014398509481988e16);
    test("-18014398509481988", -1.8014398509481988e16);
    test(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858368", 1.7976931348623157e308);
    test(
        "-17976931348623157081452742373170435679807056752584499659891747680315726078002853876058955\
        8632766878171540458953514382464234321326889464182768467546703537516986049910576551282076245\
        4900903893289440758685084551339423045832369032229481658085593321233482747978262041447231687\
        38177180919299881250404026184124858368", -1.7976931348623157e308);
}

#[test]
fn f64_exact_from_rational_fail() {
    assert_panic!(f64::exact_from(
        Rational::from_str("18014398509481983").unwrap()
    ));
    assert_panic!(f64::exact_from(
        Rational::from_str("-18014398509481983").unwrap()
    ));
    assert_panic!(f64::exact_from(
        Rational::from_str("18014398509481985").unwrap()
    ));
    assert_panic!(f64::exact_from(
        Rational::from_str("-18014398509481985").unwrap()
    ));
    assert_panic!(f64::exact_from(
        Rational::from_str("18014398509481986").unwrap()
    ));
    assert_panic!(f64::exact_from(
        Rational::from_str("-18014398509481986").unwrap()
    ));
    assert_panic!(f64::exact_from(
        Rational::from_str("18014398509481987").unwrap()
    ));
    assert_panic!(f64::exact_from(
        Rational::from_str("-18014398509481987").unwrap()
    ));
    assert_panic!(f64::exact_from(Rational::from_str(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858367").unwrap()));
    assert_panic!(f64::exact_from(Rational::from_str(
        "-17976931348623157081452742373170435679807056752584499659891747680315726078002853876058955\
        8632766878171540458953514382464234321326889464182768467546703537516986049910576551282076245\
        4900903893289440758685084551339423045832369032229481658085593321233482747978262041447231687\
        38177180919299881250404026184124858367").unwrap()));
    assert_panic!(f64::exact_from(Rational::from_str(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858369").unwrap()));
    assert_panic!(f64::exact_from(Rational::from_str(
        "-17976931348623157081452742373170435679807056752584499659891747680315726078002853876058955\
        8632766878171540458953514382464234321326889464182768467546703537516986049910576551282076245\
        4900903893289440758685084551339423045832369032229481658085593321233482747978262041447231687\
        38177180919299881250404026184124858369").unwrap()));
    assert_panic!(f64::exact_from(Rational::from_str("1/3").unwrap()));
    assert_panic!(f64::exact_from(Rational::from_str("-1/3").unwrap()));
}

#[test]
fn f64_exact_from_rational_ref_fail() {
    assert_panic!(f64::exact_from(
        &Rational::from_str("18014398509481983").unwrap()
    ));
    assert_panic!(f64::exact_from(
        &Rational::from_str("-18014398509481983").unwrap()
    ));
    assert_panic!(f64::exact_from(
        &Rational::from_str("18014398509481985").unwrap()
    ));
    assert_panic!(f64::exact_from(
        &Rational::from_str("-18014398509481985").unwrap()
    ));
    assert_panic!(f64::exact_from(
        &Rational::from_str("18014398509481986").unwrap()
    ));
    assert_panic!(f64::exact_from(
        &Rational::from_str("-18014398509481986").unwrap()
    ));
    assert_panic!(f64::exact_from(
        &Rational::from_str("18014398509481987").unwrap()
    ));
    assert_panic!(f64::exact_from(
        &Rational::from_str("-18014398509481987").unwrap()
    ));
    assert_panic!(f64::exact_from(&Rational::from_str(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858367").unwrap()));
    assert_panic!(f64::exact_from(&Rational::from_str(
        "-17976931348623157081452742373170435679807056752584499659891747680315726078002853876058955\
        8632766878171540458953514382464234321326889464182768467546703537516986049910576551282076245\
        4900903893289440758685084551339423045832369032229481658085593321233482747978262041447231687\
        38177180919299881250404026184124858367").unwrap()));
    assert_panic!(f64::exact_from(&Rational::from_str(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858369").unwrap()));
    assert_panic!(f64::exact_from(&Rational::from_str(
        "-17976931348623157081452742373170435679807056752584499659891747680315726078002853876058955\
        8632766878171540458953514382464234321326889464182768467546703537516986049910576551282076245\
        4900903893289440758685084551339423045832369032229481658085593321233482747978262041447231687\
        38177180919299881250404026184124858369").unwrap()));
    assert_panic!(f64::exact_from(&Rational::from_str("1/3").unwrap()));
    assert_panic!(f64::exact_from(&Rational::from_str("-1/3").unwrap()));
}

#[test]
fn test_f32_convertible_from_rational() {
    let test = |s: &str, out| {
        let u = Rational::from_str(s).unwrap();
        assert_eq!(f32::convertible_from(&u), out);
        assert_eq!(f32::convertible_from(u), out);
    };
    test("3", true);
    test("-3", true);
    test("123", true);
    test("-123", true);
    test("0", true);
    test("1000000000", true);
    test("-1000000000", true);
    test("16777216", true);
    test("-16777216", true);
    test("16777218", true);
    test("-16777218", true);
    test("16777217", false);
    test("-16777217", false);
    test("33554432", true);
    test("-33554432", true);
    test("33554436", true);
    test("-33554436", true);
    test("33554433", false);
    test("-33554433", false);
    test("33554434", false);
    test("-33554434", false);
    test("33554435", false);
    test("-33554435", false);
    test("340282346638528859811704183484516925439", false);
    test("-340282346638528859811704183484516925439", false);
    test("340282346638528859811704183484516925440", true);
    test("-340282346638528859811704183484516925440", true);
    test("340282346638528859811704183484516925441", false);
    test("-340282346638528859811704183484516925441", false);
    test(
        "10000000000000000000000000000000000000000000000000000",
        false,
    );
    test(
        "-10000000000000000000000000000000000000000000000000000",
        false,
    );
    test("1/3", false);
    test("-1/3", false);
}

#[test]
fn test_f64_convertible_from_rational() {
    let test = |s: &str, out| {
        let u = Rational::from_str(s).unwrap();
        assert_eq!(f64::convertible_from(&u), out);
        assert_eq!(f64::convertible_from(u), out);
    };
    test("3", true);
    test("-3", true);
    test("123", true);
    test("-123", true);
    test("0", true);
    test("1000000000", true);
    test("-1000000000", true);
    test("9007199254740992", true);
    test("-9007199254740992", true);
    test("9007199254740994", true);
    test("-9007199254740994", true);
    test("9007199254740993", false);
    test("-9007199254740993", false);
    test("18014398509481984", true);
    test("-18014398509481984", true);
    test("18014398509481988", true);
    test("-18014398509481988", true);
    test("18014398509481985", false);
    test("-18014398509481985", false);
    test("18014398509481986", false);
    test("-18014398509481986", false);
    test("18014398509481987", false);
    test("-18014398509481987", false);
    test(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858367", false);
    test(
        "-17976931348623157081452742373170435679807056752584499659891747680315726078002853876058955\
        8632766878171540458953514382464234321326889464182768467546703537516986049910576551282076245\
        4900903893289440758685084551339423045832369032229481658085593321233482747978262041447231687\
        38177180919299881250404026184124858367", false);
    test(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858368", true);
    test(
        "-17976931348623157081452742373170435679807056752584499659891747680315726078002853876058955\
        8632766878171540458953514382464234321326889464182768467546703537516986049910576551282076245\
        4900903893289440758685084551339423045832369032229481658085593321233482747978262041447231687\
        38177180919299881250404026184124858368", true);
    test(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858369", false);
    test(
        "-17976931348623157081452742373170435679807056752584499659891747680315726078002853876058955\
        8632766878171540458953514382464234321326889464182768467546703537516986049910576551282076245\
        4900903893289440758685084551339423045832369032229481658085593321233482747978262041447231687\
        38177180919299881250404026184124858369", false);

    test("1/3", false);
    test("-1/3", false);
}

#[allow(clippy::trait_duplication_in_bounds)]
fn float_rounding_from_rational_properties_helper<
    T: for<'a> ConvertibleFrom<&'a Integer>
        + for<'a> ConvertibleFrom<&'a Rational>
        + PrimitiveFloat
        + RoundingFrom<Rational>
        + for<'a> RoundingFrom<&'a Integer>
        + for<'a> RoundingFrom<&'a Rational>,
>()
where
    Rational: TryFrom<T>,
{
    rational_rounding_mode_pair_gen_var_5::<T>().test_properties(|(n, rm)| {
        let f = T::rounding_from(&n, rm);
        let neg_f = if n == 0 { T::ZERO } else { -f };
        assert_eq!(NiceFloat(T::rounding_from(-&n, -rm)), NiceFloat(neg_f));
        assert_eq!(NiceFloat(T::rounding_from(n, rm)), NiceFloat(f));
    });

    rational_gen_var_4::<T>().test_properties(|n| {
        let f = T::rounding_from(&n, RoundingMode::Exact);
        assert_eq!(
            NiceFloat(f),
            NiceFloat(T::rounding_from(&n, RoundingMode::Floor))
        );
        assert_eq!(
            NiceFloat(f),
            NiceFloat(T::rounding_from(&n, RoundingMode::Ceiling))
        );
        assert_eq!(
            NiceFloat(f),
            NiceFloat(T::rounding_from(&n, RoundingMode::Down))
        );
        assert_eq!(
            NiceFloat(f),
            NiceFloat(T::rounding_from(&n, RoundingMode::Up))
        );
        assert_eq!(
            NiceFloat(f),
            NiceFloat(T::rounding_from(&n, RoundingMode::Nearest))
        );
        assert_eq!(Rational::exact_from(f), n);
    });

    rational_gen_var_5::<T>().test_properties(|n| {
        let f_below = T::rounding_from(&n, RoundingMode::Floor);
        let f_above = f_below.next_higher();
        if f_below.is_finite() {
            assert!(Rational::exact_from(f_below) < n);
        }
        if f_above.is_finite() {
            assert!(Rational::exact_from(f_above) > n);
        }
        assert_eq!(
            NiceFloat(f_above),
            NiceFloat(T::rounding_from(&n, RoundingMode::Ceiling))
        );
        if n >= 0 {
            assert_eq!(
                NiceFloat(f_below),
                NiceFloat(T::rounding_from(&n, RoundingMode::Down))
            );
            assert_eq!(
                NiceFloat(f_above),
                NiceFloat(T::rounding_from(&n, RoundingMode::Up))
            );
        } else {
            assert_eq!(
                NiceFloat(f_above),
                NiceFloat(T::rounding_from(&n, RoundingMode::Down))
            );
            assert_eq!(
                NiceFloat(f_below),
                NiceFloat(T::rounding_from(&n, RoundingMode::Up))
            );
        }
        let f_nearest = T::rounding_from(&n, RoundingMode::Nearest);
        assert_eq!(
            NiceFloat(T::rounding_from(&n, RoundingMode::Nearest)),
            NiceFloat(f_nearest)
        );
        assert!(
            NiceFloat(f_nearest) == NiceFloat(f_below)
                || NiceFloat(f_nearest) == NiceFloat(f_above)
        );
        if f_below.is_finite() && f_above.is_finite() {
            let below_diff = &n - Rational::exact_from(f_below);
            let above_diff = Rational::exact_from(f_above) - &n;
            if NiceFloat(f_nearest) == NiceFloat(f_below) {
                assert!(below_diff <= above_diff);
            } else {
                assert!(below_diff >= above_diff);
            }
        }
    });

    rational_gen_var_6::<T>().test_properties(|n| {
        let floor = T::rounding_from(&n, RoundingMode::Floor);
        let ceiling = floor.next_higher();
        let nearest = T::rounding_from(&n, RoundingMode::Nearest);
        assert_eq!(
            NiceFloat(nearest),
            NiceFloat(if floor.to_bits().even() {
                floor
            } else {
                ceiling
            })
        );
    });

    integer_rounding_mode_pair_gen_var_1::<T>().test_properties(|(n, rm)| {
        let r: Rational = ExactFrom::exact_from(&n);
        assert_eq!(
            NiceFloat(T::rounding_from(r, rm)),
            NiceFloat(T::rounding_from(&n, rm)),
        );
    });
}

#[test]
fn float_rounding_from_rational_properties() {
    apply_fn_to_primitive_floats!(float_rounding_from_rational_properties_helper);

    let max = Rational::exact_from(f32::MAX_FINITE);
    rational_gen().test_properties(|x| {
        if x.lt_abs(&max) {
            let f = f32::rounding_from(&x, RoundingMode::Down);
            if NiceFloat(f) != NiceFloat(-0.0) {
                assert_eq!(NiceFloat(f), NiceFloat(rug::Rational::from(&x).to_f32()));
            }
        }
        assert_eq!(
            NiceFloat(f64::rounding_from(&x, RoundingMode::Down)),
            NiceFloat(rug::Rational::from(&x).to_f64())
        );
    });
}

#[allow(clippy::trait_duplication_in_bounds)]
fn float_try_from_rational_properties_helper<
    T: TryFrom<Rational, Error = PrimitiveFloatFromRationalError>
        + for<'a> TryFrom<&'a Integer>
        + for<'a> TryFrom<&'a Rational, Error = PrimitiveFloatFromRationalError>
        + for<'a> ConvertibleFrom<&'a Rational>
        + PrimitiveFloat
        + for<'a> RoundingFrom<&'a Rational>,
>()
where
    Natural: TryFrom<T, Error = NaturalFromPrimitiveFloatError>,
    Rational: TryFrom<T>,
{
    rational_gen().test_properties(|n| {
        let of = T::try_from(&n);
        assert_eq!(
            T::try_from(n.clone()).map(NiceFloat),
            of.map(|f| NiceFloat(f))
        );
        assert_eq!(
            T::try_from(-&n).map(NiceFloat),
            of.map(|f| NiceFloat(if n == 0 { T::ZERO } else { -f }))
        );
    });

    rational_gen_var_4::<T>().test_properties(|n| {
        let f = T::exact_from(&n);
        assert_eq!(
            NiceFloat(f),
            NiceFloat(T::rounding_from(&n, RoundingMode::Exact))
        );
        assert_eq!(Rational::exact_from(f), n);
    });

    rational_gen_var_5::<T>().test_properties(|n| {
        assert!(T::try_from(n).is_err());
    });

    rational_gen_var_6::<T>().test_properties(|n| {
        assert!(T::try_from(n).is_err());
    });

    integer_gen().test_properties(|n| {
        if let Ok(f) = T::try_from(&n) {
            let rn: Rational = From::from(&n);
            assert_eq!(NiceFloat(f), NiceFloat(T::exact_from(rn)));
        }
    });

    integer_gen_var_1::<T>().test_properties(|n| {
        let rn: Rational = From::from(&n);
        assert_eq!(NiceFloat(T::exact_from(&n)), NiceFloat(T::exact_from(rn)));
    });
}

#[test]
fn float_try_from_rational_properties() {
    apply_fn_to_primitive_floats!(float_try_from_rational_properties_helper);
}

#[allow(clippy::trait_duplication_in_bounds)]
fn float_convertible_from_rational_properties_helper<
    T: ConvertibleFrom<Rational>
        + for<'a> ConvertibleFrom<&'a Integer>
        + for<'a> ConvertibleFrom<&'a Rational>
        + PrimitiveFloat,
>()
where
    Rational: TryFrom<T>,
{
    rational_gen().test_properties(|n| {
        assert_eq!(T::convertible_from(&n), T::convertible_from(-n));
    });

    rational_gen_var_4::<T>().test_properties(|n| {
        assert!(T::convertible_from(n));
    });

    rational_gen_var_5::<T>().test_properties(|n| {
        assert!(!T::convertible_from(n));
    });

    rational_gen_var_6::<T>().test_properties(|n| {
        assert!(!T::convertible_from(n));
    });

    integer_gen().test_properties(|n| {
        let rn: Rational = ExactFrom::exact_from(&n);
        assert_eq!(T::convertible_from(&n), T::convertible_from(&rn));
    });
}

#[test]
fn float_convertible_from_rational_properties() {
    apply_fn_to_primitive_floats!(float_convertible_from_rational_properties_helper);
}
