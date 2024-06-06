# API Documentation

- [Concentrated Liquidity Endpoint](#concentrated-liquidity-endpoint)
- [Calculate Greeks Endpoint](#calculate-greeks-endpoint)
- [Squeeth Endpoint](#squeeth-endpoint)

## Concentrated Liquidity Endpoint

The `/concentrated-liquidity` endpoint calculates risk measures (greeks) for a concentrated liquidity position.

### Input Parameters
- `p_a` (f32): Lower price range of the position
- `p_b` (f32): Upper price range of the position
- `r_a` (f32): Reserves of token A
- `r_b` (f32): Reserves of token B
- `p` (f32): Current price of the asset

### Output
- `virtual_liquidity` (f32): Effective liquidity within the price range
- `concentrated_delta` (f32): Sensitivity of the position's value to asset price changes
- `concentrated_gamma` (f32): Rate of change of delta with respect to asset price changes

### Example
```bash
curl -X POST -H "Content-Type: application/json" -d '{"p_a":3000.0,"p_b":3500.0,"r_a":10.0,"r_b":20.0,"p":3200.0}' http://localhost:8080/concentrated_liquidity
```

## Calculate Greeks Endpoint

The `/calculate_greeks` endpoint calculates various risk measures (greeks) for options contracts.

### Input Parameters
- `s0` (f64): Current price of the underlying asset
- `x` (f64): Strike price of the option
- `t` (f64): Time to expiration of the option (in years)
- `r` (f64): Risk-free interest rate
- `q` (f64): Dividend yield of the underlying asset
- `sigma` (f64): Volatility of the underlying asset
- `v` (f64): Current price of the option

### Output
- `delta_call` (f64): Delta of a call option
- `delta_put` (f64): Delta of a put option
- `lambda_call` (f64): Lambda (elasticity) of a call option
- `lambda_put` (f64): Lambda (elasticity) of a put option
- `rho_call` (f64): Rho of a call option
- `rho_put` (f64): Rho of a put option
- `theta_call` (f64): Theta of a call option
- `theta_put` (f64): Theta of a put option
- `vega` (f64): Vega of the option
- `gamma` (f64): Gamma of the option

### Example
```bash
curl -X POST -H "Content-Type: application/json" -d '{"s0":100.0,"x":100.0,"t":1.0,"r":0.05,"q":0.0,"sigma":0.2,"v":10.0}' http://localhost:8080/calculate_greeks
```

## Squeeth Endpoint
The /squeeth endpoint calculates risk measures (greeks) for a "squeeth" position (Ethereum power perpetual contract).

### Input Parameters
- `eth_price` (f64): Current price of Ethereum (ETH)
- `normalization_factor` (f64): Normalization factor for the squeeth position
- `iv` (f64): Implied volatility of the squeeth position

### Output
- `sqth_to_usd` (f64): Value of the squeeth position in USD
- `sqth_delta` (f64): Delta of the squeeth position
- `sqth_gamma` (f64): Gamma of the squeeth position
- `sqth_theta` (f64): Theta of the squeeth position
- `sqth_vega` (f64): Vega of the squeeth position

### Example
```bash
curl -X POST -H "Content-Type: application/json" -d '{"eth_price":3000.0,"normalization_factor":0.5,"iv":0.8}' http://localhost:8080/squeeth
```