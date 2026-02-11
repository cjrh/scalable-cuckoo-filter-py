# scalable-cuckoo-filter-py

Python wrapper for [sile/scalable_cuckoo_filter](https://github.com/sile/scalable_cuckoo_filter)

See the crate that this wraps: https://docs.rs/scalable_cuckoo_filter/

This can be used as a probabilistic data structure for set membership tests. 
A cuckoo filter is similar to a Bloom filter, but it supports deletions.

This implementation is scalable, meaning that it can grow and shrink 
dynamically while maintinaing a low false positive rate.

## Install

> **The package name is different to the import name**. The install name is `cjrh-scalable-cuckoo-filter` but the import name is `scf`. This is because the package name must be unique on PyPI, and `scalable-cuckoo-filter` was already taken.

Install:

```bash
$ pip install cjrh-scalable-cuckoo-filter
```

Import:

```
import scf
from scf import PyScalableCuckooFilter
```

## Usage

Create the filter and add a few strings:

```python
from scf import PyScalableCuckooFilter


scf = PyScalableCuckooFilter(1000, 0.001)
scf.insert("hello")
scf.insert("world")
```

Now we can do membership tests:

```python
assert scf.might_contain("hello")
assert scf.might_contain("world")
assert not scf.might_contain("foo")
```

We can remove entries:

```python 
scf.remove("hello")
assert not scf.might_contain("hello")
```

We can save the filter to a file:

```python 
scf.write_to_file("filter.bin")
```

And load it back:

```python 
scf = PyScalableCuckooFilter.read_from_file("filter.bin")
assert scf.might_contain("world")
```

## Types

These are the the types that can be used with the filter:
- `str`
- `bytes`
- `bytearray`
- `int`
- `float`
- `bool`
- Iterables of the above types, with nesting.

For example, you can insert a list of integers:

```python
scf = PyScalableCuckooFilter(1000, 0.001)
scf.insert([1, 2, 3])  # list
scf.insert([1, (2, [3])])  # list(tuple(list)))
```

However, note that all iterables will hash the same as each other. For example, 
`["foo", "bar"]` and `("foo", "bar")` will be regarded as the same key.
In fact, any iterable with the same elements will hash the same.

```python
scf = PyScalableCuckooFilter(1000, 0.001)
scf.insert(range(3))  # iterable
assert scf.might_contain((0, 1, 2))  # tuple
```
