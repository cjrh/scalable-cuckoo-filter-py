import pytest
from scf import PyScalableCuckooFilter


def test_create():
    scf = PyScalableCuckooFilter(1000, 0.001)
    print(scf)


@pytest.mark.parametrize("value", [
    pytest.param("hello", id="str"),
    pytest.param(123, id="int"),
    pytest.param([1, 2, 3], id="list"),
    pytest.param((1, 2, 3), id="tuple"),
    # pytest.param({"a": 1, "b": 2}, id="dict"),
    # pytest.param(1.23, id="float"),
    pytest.param(True, id="bool"),
])
def test_types(value):
    scf = PyScalableCuckooFilter(1000, 0.001)
    assert value not in scf
    scf.insert(value)
    assert value in scf
