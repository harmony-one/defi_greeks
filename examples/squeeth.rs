// extern crate greeks;

const ETH_PRICE: f64 = 3500.0;
const NORMALIZATION_FACTOR: f64 = 0.8;
const IV: f64 = 0.9;

fn main() {
    let sqth_delta = defi_greeks_lib::sqth_delta(ETH_PRICE, NORMALIZATION_FACTOR, IV);
    let sqth_gamma = defi_greeks_lib::sqth_gamma(NORMALIZATION_FACTOR, IV);
    let sqth_theta = defi_greeks_lib::sqth_theta(ETH_PRICE, NORMALIZATION_FACTOR, IV);
    let sqth_vega = defi_greeks_lib::sqth_vega(ETH_PRICE, NORMALIZATION_FACTOR, IV);

    println!("Delta: {:2.2}", sqth_delta);
    println!("Gamma: {:2}", sqth_gamma);
    println!("Theta: {:2.2}", sqth_theta);
    println!("Vega: {:2.2}", sqth_vega);
}
