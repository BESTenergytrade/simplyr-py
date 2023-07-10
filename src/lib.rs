use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3::types::{PyDict, PyList};

use std::{time::Instant};

// Import the market matching algorithms and structs
use simplyr_lib::{pay_as_bid_matching, MarketInput, MarketOutput, Order};


#[pymodule]
fn simplyr_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(run, m)?)?;
    Ok(())
}

//Todo: orders_string: &PyList
#[pyfunction]
fn run(py: Python, algorithm: String, orders_string: String) -> PyResult<Py<PyList>> {
//fn run(algorithm: String, orders_string: String) -> PyResult<String> {
    println!("Run Matching");

    let now = Instant::now();

    let orders: Vec<Order> = serde_json::from_str(&orders_string).unwrap();
    let time_step = &orders[0].time_slot;

    let market_input = MarketInput { orders: orders.clone() };

    let matches: MarketOutput = pay_as_bid_matching(&market_input);

    let elapsed = now.elapsed();
    println!("Time Elapsed for PayAsBid Algorithm is: {:.2?}", elapsed);
    println!("{:?}", matches);

    // Convert MarketOutput to Python List of Dict representing matches
    let pyMatches = PyList::new(py, 
        matches.matches.into_iter().map(|mtch| {
            let dict = PyDict::new(py);
            dict.set_item("time", mtch.energy_kwh);
            dict.set_item("bid_id", mtch.bid_id);
            dict.set_item("ask_id", mtch.ask_id);
            dict.set_item("bid_actor", "");
            dict.set_item("ask_actor", "");
            dict.set_item("bid_cluster", 0);
            dict.set_item("ask_cluster", 0);
            dict.set_item("energy", mtch.energy_kwh);
            dict.set_item("price", mtch.price_euro_per_kwh);
            dict.set_item("included_grid_fee", 0);
            dict
        })
    );

    Ok(pyMatches.into())
}
