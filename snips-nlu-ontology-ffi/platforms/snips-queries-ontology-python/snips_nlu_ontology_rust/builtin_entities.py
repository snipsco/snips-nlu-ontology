# coding=utf-8
from __future__ import absolute_import
from __future__ import division
from __future__ import print_function
from __future__ import unicode_literals

import os
from builtins import object
from builtins import range
from ctypes import *
from glob import glob

dylib_dir = os.path.join(os.path.dirname(__file__), "dylib")
dylib_path = glob(os.path.join(dylib_dir, "libsnips_nlu_ontology*"))[0]
lib = cdll.LoadLibrary(dylib_path)

_ALL_LANGUAGES = None
_ALL_BUILTIN_ENTITIES = None


class CArrayString(Structure):
    _fields_ = [
        ("data", POINTER(c_char_p)),
        ("size", c_int)
    ]


def get_all_languages():
    global _ALL_LANGUAGES
    if _ALL_LANGUAGES is None:
        lib.nlu_ontology_supported_languages.restype = POINTER(CArrayString)
        array_ptr = lib.nlu_ontology_supported_languages()
        size = array_ptr.contents.size
        _ALL_LANGUAGES = set(
            array_ptr.contents.data[i].decode("utf8") for i in range(size))
    return _ALL_LANGUAGES


def get_all_builtin_entities():
    global _ALL_BUILTIN_ENTITIES
    if _ALL_BUILTIN_ENTITIES is None:
        lib.nlu_ontology_all_builtin_entities.restype = POINTER(CArrayString)
        array_ptr = lib.nlu_ontology_all_builtin_entities()
        size = array_ptr.contents.size
        _ALL_BUILTIN_ENTITIES = set(
            array_ptr.contents.data[i].decode("utf8") for i in range(size))
    return _ALL_BUILTIN_ENTITIES


class BuiltinEntityParser(object):
    """Extract builtin entities

    Args:
        language (str): Language (ISO code) of the builtin entity parser
    """

    def __init__(self, language):
        self._parser = pointer(c_void_p())
        exit_code = lib.nlu_ontology_create_rustling_parser(
            language, byref(self._parser))
        if exit_code:
            raise ImportError('Something wrong happened while creating the '
                              'intent parser. See stderr.')

    def __del__(self):
        lib.nlu_ontology_destroy_rustling_parser(self._parser)

    def parse(self, text, scope=None):
        """Extract builtin entities from *text*

        Args:
            text (str): Input
            scope (list of str, optional): List of builtin entity labels. If
            defined, the parser will extract entities using the provided scope
            instead of the entire scope of all entities. This allows to look
            for specify builtin entity kinds

        Returns:
            list of dict: The list of extracted entities
        """
        if scope is None:
            scope = self.supported_entities()
        return [{
            "range": [0, len(text)],
            "value": {
                "kind": "Number",
                "value": 2
            },
            "entity": "snips/number"
        }]

    def supported_entities(self):
        """Lists the builtin entities supported in the parser's language

        Returns:
              list of str: the list of entity labels
        """
        return [
            "snips/number",
            "snips/datetime",
            "snips/duration",
            "snips/temperature",
            "snips/ordinal",
            "snips/percentage",
            "snips/amountOfMoney"
        ]
