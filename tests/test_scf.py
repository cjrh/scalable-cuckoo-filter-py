import pytest
from scf import PyScalableCuckooFilter


def test_create():
    scf = PyScalableCuckooFilter(1000, 0.001)
    print(scf)


@pytest.mark.parametrize("value", [
    pytest.param("hello", id="str"),
    pytest.param(b"hello", id="bytes"),
    pytest.param(123, id="int"),
    pytest.param([1, 2, 3], id="list"),
    pytest.param([1, "a"], id="list"),
    pytest.param((1, 2, 3), id="tuple"),
    # pytest.param({"a": 1, "b": 2}, id="dict"),
    pytest.param(1.23, id="float"),
    pytest.param((1.23, 4.25), id="float"),
    pytest.param(True, id="bool"),
    pytest.param(bytearray([1, 2, 3]), id="bytearray"),
    pytest.param([1, [2, [3]]], id="nested"),
])
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
