use actix_web::{web, App, HttpServer};
use serde::{Deserialize, Serialize};

// Import the necessary functions and structs from your library
use crate::greeks::*;
use crate::greeks::first::{delta_call, delta_put, lambda_call, lambda_put, rho_call, rho_put, theta_call, theta_put, vega};
use crate::greeks::second::gamma;
use crate::greeks::squeeks::{sqth_delta, sqth_gamma, sqth_theta, sqth_to_usd, sqth_vega};
use crate::greeks::concentrated_liquidity::{concentrated_delta, concentrated_gamma, virtual_liquidity};

// ... (previous code for PriceRequest, PriceResponse, price route, ValueRequest, ValueResponse, value route)

#[derive(Deserialize)]
struct GreeksRequest {
    s0: f64,
    x: f64,
    t: f64,
    r: f64,
    q: f64,
    sigma: f64,
    v: f64,
}

#[derive(Serialize)]
struct GreeksResponse {
    delta_call: f64,
    delta_put: f64,
    lambda_call: f64,
    lambda_put: f64,
    rho_call: f64,
    rho_put: f64,
    theta_call: f64,
    theta_put: f64,
    vega: f64,
    gamma: f64,
}

// Define the API route for calculating greeks
#[actix_web::post("/greeks")]
async fn greeks(req: web::Json<GreeksRequest>) -> web::Json<GreeksResponse> {
    let delta_call = delta_call(req.s0, req.x, req.t, req.r, req.q, req.sigma);
    let delta_put = delta_put(req.s0, req.x, req.t, req.r, req.q, req.sigma);
    let lambda_call = lambda_call(req.s0, req.x, req.t, req.r, req.q, req.sigma, req.v);
    let lambda_put = lambda_put(req.s0, req.x, req.t, req.r, req.q, req.sigma, req.v);
    let rho_call = rho_call(req.s0, req.x, req.t, req.r, req.q, req.sigma);
    let rho_put = rho_put(req.s0, req.x, req.t, req.r, req.q, req.sigma);
    let theta_call = theta_call(req.s0, req.x, req.t, req.r, req.q, req.sigma, 365.0);
    let theta_put = theta_put(req.s0, req.x, req.t, req.r, req.q, req.sigma, 365.0);
    let vega = vega(req.s0, req.x, req.t, req.r, req.q, req.sigma);
    let gamma = gamma(req.s0, req.x, req.t, req.r, req.q, req.sigma);

    let response = GreeksResponse {
        delta_call,
        delta_put,
        lambda_call,
        lambda_put,
        rho_call,
        rho_put,
        theta_call,
        theta_put,
        vega,
        gamma,
    };

    web::Json(response)
}

#[derive(Deserialize)]
struct SqueethRequest {
    eth_price: f64,
    normalization_factor: f64,
    iv: f64,
}

#[derive(Serialize)]
struct SqueethResponse {
    sqth_to_usd: f64,
    sqth_delta: f64,
    sqth_gamma: f64,
    sqth_theta: f64,
    sqth_vega: f64,
}

// Define the API route for calculating squeeth greeks
#[actix_web::post("/squeeth")]
async fn squeeth(req: web::Json<SqueethRequest>) -> web::Json<SqueethResponse> {
    let sqth_to_usd = sqth_to_usd(req.eth_price, req.normalization_factor, req.iv);
    let sqth_delta = sqth_delta(req.eth_price, req.normalization_factor, req.iv);
    let sqth_gamma = sqth_gamma(req.normalization_factor, req.iv);
    let sqth_theta = sqth_theta(req.eth_price, req.normalization_factor, req.iv);
    let sqth_vega = sqth_vega(req.eth_price, req.normalization_factor, req.iv);

    let response = SqueethResponse {
        sqth_to_usd,
        sqth_delta,
        sqth_gamma,
        sqth_theta,
        sqth_vega,
    };

    web::Json(response)
}

#[derive(Deserialize)]
struct ConcentratedLiquidityRequest {
    p_a: f32,
    p_b: f32,
    r_a: f32,
    r_b: f32,
    p: f32,
}

#[derive(Serialize)]
struct ConcentratedLiquidityResponse {
    virtual_liquidity: f32,
    concentrated_delta: f32,
    concentrated_gamma: f32,
}

// Define the API route for calculating concentrated liquidity greeks
#[actix_web::post("/concentrated-liquidity")]
async fn concentrated_liquidity(req: web::Json<ConcentratedLiquidityRequest>) -> web::Json<ConcentratedLiquidityResponse> {
    let virtual_liquidity = virtual_liquidity(req.p_a, req.p_b, req.r_a, req.r_b);
    let concentrated_delta = concentrated_delta(virtual_liquidity, req.p, req.p_b);
    let concentrated_gamma = concentrated_gamma(virtual_liquidity, req.p);

    let response = ConcentratedLiquidityResponse {
        virtual_liquidity,
        concentrated_delta,
        concentrated_gamma,
    };

    web::Json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // .service(price)
            // .service(value)
            .service(greeks)
            .service(squeeth)
            .service(concentrated_liquidity)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}