use pyo3::prelude::*;
use rand::Rng;
use scalable_cuckoo_filter::ScalableCuckooFilter;
use std::borrow::Cow;
use std::cmp::Ordering;
use std::hash::Hash;
use std::io;
use std::path::PathBuf;

use ordered_float::OrderedFloat;

#[derive(Hash, Debug)]
struct MyNotNan(OrderedFloat<f64>);

impl<'py> FromPyObject<'py> for MyNotNan {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        let value: f64 = ob.extract()?;
        let nn = OrderedFloat(value);
        Ok(MyNotNan(nn))
    }
}

#[derive(FromPyObject, Hash, Debug)]
enum Value {
    String(String),
    Vec(Vec<Value>),
    Bytes(Vec<u8>),
    Int(i64),
    Float(MyNotNan),
    Bool(bool),
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
            inner: ScalableCuckooFilter::new(initial_capacity_hint, false_positive_probability),
        }
    }

    fn __len__(&self) -> usize {
        self.inner.len()
    }

    #[staticmethod]
    fn debug_value(item: Value) -> String {
        format!("{:?}", item)
    }

    fn __repr__(&self) -> String {
        format!(
            "PyScalableCuckooFilter(len={}, capacity={}, bits={}, fpp={})",
            self.inner.len(),
            self.inner.capacity(),
            self.inner.bits(),
            self.inner.false_positive_probability()
        )
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

    /// This name best expresses the probabilistic nature of the filter.
    /// If this returns `true`, the item might be in the filter. If this returns `false`,
    /// the item is definitely not in the filter.
    fn might_contain(&self, item: Value) -> bool {
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

    fn serialize(&self) -> PyResult<Cow<[u8]>> {
        bincode::serialize(&self.inner)
            .map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                    "Serialization error: {}",
                    e
                ))
            })
            .map(Cow::Owned)
    }

    #[staticmethod]
    fn deserialize(serialized: Vec<u8>) -> Self {
        PyScalableCuckooFilter {
            inner: bincode::deserialize(&serialized).unwrap(),
        }
    }

    fn write_to_file(&self, path: PathBuf) -> PyResult<()> {
        let tmpfilename = path.with_extension("tmp");
        let file = std::fs::File::create(&tmpfilename)?;
        bincode::serialize_into(file, &self.inner).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Serialization error: {}", e))
        })?;
        // Renames are atomic on Unix
        std::fs::rename(&tmpfilename, &path)?;
        Ok(())
    }

    #[staticmethod]
    fn read_from_file(path: PathBuf) -> PyResult<Self> {
        let file = std::fs::File::open(path)?;
        let inner = bincode::deserialize_from(file).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Deserialization error: {}", e))
        })?;
        Ok(PyScalableCuckooFilter { inner })
    }
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
