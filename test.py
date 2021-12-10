#!/usr/bin/env python

from ctypes import byref, cdll, create_string_buffer
from sys import argv

args = " ".join(argv[1:])
STRING = create_string_buffer(str("x" * 98).encode("utf-8") if not args else args.encode("utf-8"))
LEN = str(len(STRING)).encode("utf-8")
OUTPUT = create_string_buffer(b"", 100)
cbl = cdll.LoadLibrary("build/libentry.so")
print("Returned: ", cbl.entry(byref(STRING), byref(create_string_buffer(LEN)), byref(OUTPUT)))
print("Output: ", OUTPUT.value.decode("utf-8"))
