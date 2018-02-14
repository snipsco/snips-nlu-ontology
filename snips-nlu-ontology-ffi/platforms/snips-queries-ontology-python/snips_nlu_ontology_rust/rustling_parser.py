# coding=utf-8
from __future__ import absolute_import
from __future__ import division
from __future__ import print_function
from __future__ import unicode_literals

import json
import os
from builtins import object, bytes
from ctypes import *
from glob import glob

dylib_path = glob(
    os.path.join(os.path.dirname(__file__), "dylib", "libsnips_nlu_ontology*"))[0]
lib = cdll.LoadLibrary(dylib_path)

# pub struct CRustlingEntity {
#     pub value: *const libc::c_char,
#     pub range_start: libc::c_int,
#     pub range_end: libc::c_int,
#     pub entity: ::CSlotValue,
#     pub entity_kind: ::CBuiltinEntityKind,
# }

(AMOUNT_OF_MONEY,
 DURATION,
 NUMBER,
 ORDINAL,
 TEMPERATURE,
 TIME,
 PERCENTAGE) = map(ctypes.c_int, xrange(7))

(DE, EN, ES, FR, KO, ZH) = map(ctypes.c_int, xrange(6))

class RustlingParser(object):
    def __init__(self, language):
        self._parser = pointer(c_void_p())
        exit_code = lib.nlu_ontology_create_rustling_parser(
            FR,
            byref(self._parser))
        if exit_code != 0:
            raise ImportError('Something wrong happened while creating the '
                              'intent parser. See stderr.')

    def __del__ (self):
        lib.nlu_ontology_destroy_rustling_parser(self._parser)

    def parse(self, sentence, filter_entity_kinds):
        pointer = c_char_p()
        lib.nlu_ontology_extract_entities(
            self._parser,
            sentence.encode("utf-8"),
            byref(pointer))

        result = string_at(pointer)

        # if data_path is None and data_zip is None:
            # raise ValueError("Please specify data_path or data_zip")

        # if data_path is not None:
            # self.data_path = data_path
            # self._engine = pointer(c_void_p())
            # exit_code = lib.nlu_engine_create_from_dir(
                # data_path.encode("utf-8"), byref(self._engine))

        # if data_zip is not None:
            # self._engine = pointer(c_void_p())
            # bytearray_type = c_char * len(data_zip)
            # exit_code = lib.nlu_engine_create_from_zip(
                # bytearray_type.from_buffer(data_zip), len(data_zip),
                # byref(self._engine))

        # if exit_code != 1:
            # raise ImportError('Something wrong happened while creating the '
                              # 'intent parser. See stderr.')


    # def __del__(self):
        # lib.nlu_engine_destroy_client(self._engine)


    # def parse(self, query):
        # pointer = c_char_p()
        # lib.nlu_engine_run_parse_into_json(
            # self._engine,
            # query.encode("utf-8"),
            # byref(pointer))
        # result = string_at(pointer)
        # lib.nlu_engine_destroy_string(pointer)

        # return json.loads(bytes(result).decode())
