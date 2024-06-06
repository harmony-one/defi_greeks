use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use defi_greeks_lib::logger::{self, Header};
use serde::{Deserialize, Serialize};
use dotenv::dotenv;
use defi_greeks_lib::config::ENVIRONMENT;

use defi_greeks_lib::concentrated_liquidity::{concentrated_delta, concentrated_gamma, virtual_liquidity};
use defi_greeks_lib::first::{delta_call, delta_put, lambda_call, lambda_put, rho_call, rho_put, theta_call, theta_put, vega};
use defi_greeks_lib::second::gamma;
use defi_greeks_lib::squeeks::{sqth_delta, sqth_gamma, sqth_theta, sqth_to_usd, sqth_vega};

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

#[actix_web::post("/calculate_greeks")]
async fn calculate_greeks(req: web::Json<GreeksRequest>) -> web::Json<GreeksResponse> {
    logger::log(Header::INFO, "Handling calculate_greeks request");
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

#[actix_web::post("/squeeth")]
async fn squeeth(req: web::Json<SqueethRequest>) -> web::Json<SqueethResponse> {
    logger::log(Header::INFO, "Handling squeeth request");
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

#[actix_web::post("/concentrated-liquidity")]
async fn concentrated_liquidity(req: web::Json<ConcentratedLiquidityRequest>) -> web::Json<ConcentratedLiquidityResponse> {
    logger::log(Header::INFO, "Handling concentrated-liquidity request");
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

#[actix_web::get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("I'm healthy")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let (ip_address, port) = if *ENVIRONMENT == "production" {
        logger::log(Header::INFO, "Production environment");
        ("0.0.0.0".to_string(), "8080".to_string())
    } else {
        logger::log(Header::INFO, "Development environment");
        ("127.0.0.1".to_string(), "8080".to_string())
    };

    let bind_address = format!("{}:{}", ip_address, port);

    HttpServer::new(|| {
        App::new()
            .service(calculate_greeks)
            .service(squeeth)
            .service(concentrated_liquidity)
            .service(health)
    })
    .bind(bind_address)?
    .run()
    .await
}