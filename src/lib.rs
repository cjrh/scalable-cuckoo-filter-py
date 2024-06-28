use pyo3::prelude::*;
use rand::Rng;
use scalable_cuckoo_filter::ScalableCuckooFilter;
use std::cmp::Ordering;
use std::io;
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;

// Required to call the `.hash` and `.finish` methods, which are defined on traits.
use std::hash::{Hash, Hasher};



// Let's make an enum that can receive a Python object and return
// the correct rust variant
// #[pyclass]
#[derive(FromPyObject, Hash)]
enum Value {
    #[pyo3(transparent, annotation = "str")]
    String(String),
    Int(i64),
    // Float(f64),
    Bool(bool),
    Vec(Vec<Value>),
}


// Using pyo3, let's make a wrapper class for the `scalable_cuckoo_filter::ScalableCuckooFilter`
// struct. This will allow us to create a Python class that wraps the Rust struct.
// The documentation is here:
// https://docs.rs/scalable_cuckoo_filter/0.3.2/scalable_cuckoo_filter/struct.ScalableCuckooFilter.html
#[pyclass(unsendable)]
pub(crate) struct PyScalableCuckooFilter {
    inner: ScalableCuckooFilter<Value>,
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

    fn __len__(&self) -> usize {
        self.inner.len()
    }

    fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    fn bits(&self) -> u64 {
        self.inner.bits()
    }

    fn false_positive_probability(&self) -> f64 {
        self.inner.false_positive_probability()
    }

    fn entries_per_bucket(&self) -> usize {
        self.inner.entries_per_bucket()
    }

    fn max_kicks(&self) -> usize {
        self.inner.max_kicks()
    }

    fn __contains__(&self, item: Value) -> bool {
        self.inner.contains(&item)
    }

    fn insert(&mut self, item: Value) {
        self.inner.insert(&item)
    }

    fn shrink_to_fit(&mut self) {
        self.inner.shrink_to_fit()
    }

    fn remove(&mut self, item: Value) -> bool {
        self.inner.remove(&item)
    }

    // Methods for serialization and deserialization

    fn serialize(&self) -> Vec<u8> {
        // Use Serde to serialize the `ScalableCuckooFilter` struct to a byte array.
        // TODO
        vec![]
    }

    // #[staticmethod]
    // fn deserialize(serialized: Vec<u8>) -> Self {
    //     // TODO
    //     // PyScalableCuckooFilter {
    //     //     inner: ScalableCuckooFilter::deserialize(&serialized),
    //     // }
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
