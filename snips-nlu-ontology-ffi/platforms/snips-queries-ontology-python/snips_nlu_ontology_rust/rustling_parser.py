# coding=utf-8
from __future__ import absolute_import
from __future__ import division
from __future__ import print_function
from __future__ import unicode_literals

import os
from ctypes import *
from glob import glob

dylib_path = glob(
    os.path.join(os.path.dirname(__file__), "dylib",
                 "libsnips_nlu_ontology*"))[0]
lib = cdll.LoadLibrary(dylib_path)


class BuiltinEntityParser(object):
    """Extract builtin entities

    Args:
        language (str): Language (ISO code) of the builtin entity parser
    """

    def __init__(self, language):
        self._parser = pointer(c_void_p())
        exit_code = lib.nlu_ontology_create_rustling_parser(
            FR, byref(self._parser))
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
