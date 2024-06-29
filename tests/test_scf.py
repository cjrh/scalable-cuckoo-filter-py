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
        pytest.param((1.23, 4.25), id="float"),
        pytest.param(True, id="bool"),
        pytest.param(bytearray([1, 2, 3]), id="bytearray"),
        pytest.param([1, [2, [3]]], id="nested"),
    ],
)
def test_types(value):
    scf = PyScalableCuckooFilter(1000, 0.001)

    # Should fail membership test
    assert value not in scf
    scf.insert(value)

    # Should pass membership test
    assert value in scf


def test_list_tuple():
    scf = PyScalableCuckooFilter(1000, 0.001)
    assert [1, 2, 3] not in scf
    assert (1, 2, 3) not in scf
    scf.insert([1, 2, 3])
    assert [1, 2, 3] in scf
    assert (1, 2, 3) in scf


def test_str_bytes():
    scf = PyScalableCuckooFilter(1000, 0.001)
    assert "hello" not in scf
    assert b"hello" not in scf

    scf.insert("hello")
    assert "hello" in scf
    assert b"hello" not in scf

    scf.insert(b"hello")
    assert b"hello" in scf

    scf.remove("hello")
    assert b"hello" in scf
    assert "hello" not in scf

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
    assert "hello" in scf

    # Now write to a file
    filename = tmpdir / "scf.bin"
    scf.write_to_file(filename)

    # We'll check the file is correct
    with open(filename, "rb") as f:
        assert f.read() == ser

    # Read it back from the file
    scf = PyScalableCuckooFilter.read_from_file(filename)
    assert "hello" in scf


def test_scaling():
    from uuid import uuid4

    scf = PyScalableCuckooFilter(100, 0.0001)
    print(scf)

    for _ in range(10000000):
        scf.insert(uuid4().hex)

    print()
    print(scf)

    ser = scf.serialize()
    print(f"Size is {len(ser) / 1000:.2f} kB")






