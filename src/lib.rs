use pyo3::prelude::*;
use rand::Rng;
use scalable_cuckoo_filter::ScalableCuckooFilter;
use std::cmp::Ordering;
use std::io;

// Using pyo3, let's make a wrapper class for the `scalable_cuckoo_filter::ScalableCuckooFilter`
// struct. This will allow us to create a Python class that wraps the Rust struct.
// The documentation is here:
// https://docs.rs/scalable_cuckoo_filter/0.3.2/scalable_cuckoo_filter/struct.ScalableCuckooFilter.html
#[pyclass(unsendable)]
pub(crate) struct PyScalableCuckooFilter {
    inner: ScalableCuckooFilter<String>,
}

/*

#[pyclass(frozen, module = "tantivy.tantivy")]
pub(crate) struct Query {
    pub(crate) inner: Box<dyn tv::query::Query>,
}

*/

// Implement the Python class methods for the `PyScalableCuckooFilter` class.
// This will allow us to create a Python class that wraps the Rust struct.
#[pymethods]
impl PyScalableCuckooFilter {
    #[new]
    fn new(initial_capacity_hint: usize, false_positive_probability: f64) -> Self {
        PyScalableCuckooFilter {
            inner: ScalableCuckooFilter::new(
                initial_capacity_hint,
                false_positive_probability,
            ),
        }
    }

    // fn insert(&mut self, item: u64) -> bool {
    //     self.inner.insert(item)
    // }

    // fn contains(&self, item: u64) -> bool {
    //     self.inner.contains(item)
    // }

    // fn remove(&mut self, item: u64) -> bool {
    //     self.inner.remove(item)
    // }
}

#[pyfunction]
fn guess_the_number() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn scf(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyScalableCuckooFilter>()?;
    m.add_function(wrap_pyfunction!(guess_the_number, m)?)?;

    Ok(())
}
