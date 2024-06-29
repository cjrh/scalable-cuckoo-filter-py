use pyo3::prelude::*;
use rand::Rng;
use scalable_cuckoo_filter::ScalableCuckooFilter;
use std::cmp::Ordering;
use std::io;
use std::hash::Hash;

use ordered_float::NotNan;


#[derive(Hash)]
struct MyNotNan(NotNan<f64>);

impl<'py> FromPyObject<'py> for MyNotNan {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        let value: f64 = ob.extract()?;
        Ok(MyNotNan(NotNan::new(value).unwrap()))
    }
}


#[derive(FromPyObject, Hash)]
enum Value {
    String(String),
    Bytes(Vec<u8>),
    Int(i64),
    Float(MyNotNan),
    Bool(bool),
    Vec(Vec<Value>),
}


// The documentation is here:
// https://docs.rs/scalable_cuckoo_filter/0.3.2/scalable_cuckoo_filter/struct.ScalableCuckooFilter.html
#[pyclass(unsendable)]
pub(crate) struct PyScalableCuckooFilter {
    inner: ScalableCuckooFilter<Value>,
}

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

    /// This is repeated here just for consistency with the
    /// other methods.
    fn contains(&self, item: Value) -> bool {
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
        use serde::Serialize;
        let serialized = bincode::serialize(&self.inner).unwrap();
        serialized
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
