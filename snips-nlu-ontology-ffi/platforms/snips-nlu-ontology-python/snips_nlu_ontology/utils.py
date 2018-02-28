import os
from _ctypes import Structure, POINTER
from contextlib import contextmanager
from ctypes import c_char_p, c_int32, cdll
from glob import glob

dylib_dir = os.path.join(os.path.dirname(__file__), "dylib")
dylib_path = glob(os.path.join(dylib_dir, "libsnips_nlu_ontology_rs*"))[0]
lib = cdll.LoadLibrary(dylib_path)


@contextmanager
def string_array_pointer(ptr):
    try:
        yield ptr
    finally:
        lib.ffi_nlu_ontology_destroy_string_array(ptr)


@contextmanager
def string_pointer(ptr):
    try:
        yield ptr
    finally:
        lib.ffi_nlu_ontology_destroy_string(ptr)


class CStringArray(Structure):
    _fields_ = [
        ("data", POINTER(c_char_p)),
        ("size", c_int32)
    ]
