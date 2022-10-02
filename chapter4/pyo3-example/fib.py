import pyo3_example
from time import time


def fib(a):
    if a <= 2:
        return 1
    return fib(a - 2) + fib(a - 1)


start = time()
_ = fib(40)
print("python: ", time() - start, "sec")

start = time()
_ = pyo3_example.fib(40)
print("rust  : ", time() - start, "sec")
