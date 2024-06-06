#![feature(test)]

extern crate test;

use self::test::Bencher;

const UNDERLYING: f64 = 64.68;
const STRIKE: f64 = 65.00;
const VOL: f64 = 0.5051;
const INTEREST_RATE: f64 = 0.0150;
const DIV_YIELD: f64 = 0.0210;
const DAYS_PER_YEAR: f64 = 365.0;
const TIME_TO_EXPIRY: f64 = 23.0 / DAYS_PER_YEAR;

#[bench]
fn delta_call_bench(b: &mut Bencher) {
    let _r = b.iter(|| {
        defi_greeks_lib::delta_call(
            UNDERLYING,
            STRIKE,
            TIME_TO_EXPIRY,
            INTEREST_RATE,
            DIV_YIELD,
            VOL,
        )
    });
}

#[bench]
fn delta_put_bench(b: &mut Bencher) {
    let _r = b.iter(|| {
        defi_greeks_lib::delta_put(
            UNDERLYING,
            STRIKE,
            TIME_TO_EXPIRY,
            INTEREST_RATE,
            DIV_YIELD,
            VOL,
        )
    });
}

#[bench]
fn rho_call_bench(b: &mut Bencher) {
    let _r = b.iter(|| {
        defi_greeks_lib::rho_call(
            UNDERLYING,
            STRIKE,
            TIME_TO_EXPIRY,
            INTEREST_RATE,
            DIV_YIELD,
            VOL,
        )
    });
}

#[bench]
fn rho_put_bench(b: &mut Bencher) {
    let _r = b.iter(|| {
        defi_greeks_lib::rho_put(
            UNDERLYING,
            STRIKE,
            TIME_TO_EXPIRY,
            INTEREST_RATE,
            DIV_YIELD,
            VOL,
        )
    });
}

#[bench]
fn theta_call_bench(b: &mut Bencher) {
    let _r = b.iter(|| {
        defi_greeks_lib::theta_call(
            UNDERLYING,
            STRIKE,
            TIME_TO_EXPIRY,
            INTEREST_RATE,
            DIV_YIELD,
            VOL,
            DAYS_PER_YEAR,
        )
    });
}

#[bench]
fn theta_put_bench(b: &mut Bencher) {
    let _r = b.iter(|| {
        defi_greeks_lib::theta_put(
            UNDERLYING,
            STRIKE,
            TIME_TO_EXPIRY,
            INTEREST_RATE,
            DIV_YIELD,
            VOL,
            DAYS_PER_YEAR,
        )
    });
}

#[bench]
fn vega_bench(b: &mut Bencher) {
    let _r = b.iter(|| {
        defi_greeks_lib::vega(
            UNDERLYING,
            STRIKE,
            TIME_TO_EXPIRY,
            INTEREST_RATE,
            DIV_YIELD,
            VOL,
        );
    });
}

#[bench]
fn gamma_bench(b: &mut Bencher) {
    let _r = b.iter(|| {
        defi_greeks_lib::gamma(
            UNDERLYING,
            STRIKE,
            TIME_TO_EXPIRY,
            INTEREST_RATE,
            DIV_YIELD,
            VOL,
        )
    });
}
