import math

import pytest
from scf import PyScalableCuckooFilter


def test_create():
    scf = PyScalableCuckooFilter(1000, 0.001)
    assert scf.is_empty()
    print(scf)


@pytest.mark.parametrize(
    "value",
    [
        pytest.param("hello", id="str"),
        pytest.param(b"hello", id="bytes"),
        pytest.param(123, id="int"),
        pytest.param([1, 2, 3], id="list"),
        pytest.param([1, "a"], id="list"),
        pytest.param((1, 2, 3), id="tuple"),
        pytest.param(1.23, id="float"),
        pytest.param(math.nan, id="float (nan)"),
        pytest.param((1.23, 4.25), id="float (seq)"),
        pytest.param((1.23, 4.25), id="float (seq, nan)"),
        pytest.param(True, id="bool"),
        pytest.param(bytearray([1, 2, 3]), id="bytearray"),
        pytest.param([1, [2, [3]]], id="nested"),
        pytest.param(range(4, 10, 2), id="iterable"),
    ],
)
def test_types(value):
    value_converted = PyScalableCuckooFilter.debug_value(value)
    print(f"Input value {value} converts to {value_converted}")

    scf = PyScalableCuckooFilter(1000, 0.001)

    # Should fail membership test
    assert not scf.might_contain(value)
    scf.insert(value)

    # Should pass membership test
    assert scf.might_contain(value)


def test_list_tuple():
    scf = PyScalableCuckooFilter(1000, 0.001)
    assert not scf.might_contain([1, 2, 3])
    assert not scf.might_contain((1, 2, 3))
    scf.insert([1, 2, 3])
    assert scf.might_contain([1, 2, 3])
    assert scf.might_contain((1, 2, 3))

    scf.insert(range(5))
    assert scf.might_contain([0, 1, 2, 3, 4])


def test_str_bytes():
    scf = PyScalableCuckooFilter(1000, 0.001)
    assert not scf.might_contain("hello")
    assert not scf.might_contain(b"hello")

    scf.insert("hello")
    assert scf.might_contain("hello")
    assert not scf.might_contain(b"hello")

    scf.insert(b"hello")
    assert scf.might_contain(b"hello")

    scf.remove("hello")
    assert not scf.might_contain("hello")
    assert scf.might_contain(b"hello")

    assert scf.false_positive_probability() == 0.001


def test_serialization(tmpdir):
    scf = PyScalableCuckooFilter(100, 0.01)
    scf.insert("hello")
    ser = scf.serialize()
    assert isinstance(ser, bytes)
    expected = (
        "01000000000000000b0000000000000004000000000000002c000000"
        "000000000500000000000000b0000000000000000000000000000000"
        "00000000000000000000000000000000000000000000000000000000"
        "00000000000000000000000000000000000000000000000000000000"
        "00000000000000000000000000000000000000000000000000000000"
        "00000000000000000000000000000000000000000000000000000000"
        "0000000000000000000000000000000000000000000000d507000000"
        "00000000000000000000000000000000000000000000000000000000"
        "00020000000000000000000000000000010000000000000064000000"
        "000000007b14ae47e17a843f04000000000000000002000000000000"
    )
    assert ser.hex() == expected

    # Read it back and test the membership
    scf = PyScalableCuckooFilter.deserialize(ser)
    assert scf.might_contain("hello")

    # Now write to a file
    filename = tmpdir / "scf.bin"
    scf.write_to_file(filename)

    # We'll check the file is correct
    with open(filename, "rb") as f:
        assert f.read() == ser

    # Read it back from the file
    scf = PyScalableCuckooFilter.read_from_file(filename)
    assert scf.might_contain("hello")


def test_scaling():
    from uuid import uuid4

    scf = PyScalableCuckooFilter(100, 0.01)
    print(scf)

    for _ in range(100000):
        scf.insert(uuid4().hex)

    print()
    print(scf)

    ser = scf.serialize()
    print(f"Size is {len(ser) / 1000:.2f} kB")






