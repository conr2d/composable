mod orderbook;
mod prelude;
mod solution;
mod solver;
mod types;

fn main() {
    let order_ = types::Order::new_random(1.0, 0.05, (50, 150), 42);
    // to be continued...
}
