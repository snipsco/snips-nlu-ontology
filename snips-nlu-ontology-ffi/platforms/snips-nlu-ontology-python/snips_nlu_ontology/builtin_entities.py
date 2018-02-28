# coding=utf-8
from __future__ import absolute_import
from __future__ import division
from __future__ import print_function
from __future__ import unicode_literals

import json
from _ctypes import pointer, byref
from builtins import object, range, str
from ctypes import c_char_p, c_void_p, c_int, string_at

from snips_nlu_ontology.utils import (
    string_array_pointer, string_pointer, CStringArray, lib)

_ALL_LANGUAGES = None
_SUPPORTED_ENTITIES = dict()
_ALL_BUILTIN_ENTITIES = None
_ONTOLOGY_VERSION = None


def get_ontology_version():
    """Get the version of the ontology"""
    global _ONTOLOGY_VERSION
    if _ONTOLOGY_VERSION is None:
        lib.ffi_nlu_ontology_version.restype = c_char_p
        _ONTOLOGY_VERSION = lib.ffi_nlu_ontology_version().decode("utf8")
    return _ONTOLOGY_VERSION


def get_all_languages():
    """Lists all the supported languages"""
    global _ALL_LANGUAGES
    if _ALL_LANGUAGES is None:
        lib.ffi_nlu_ontology_supported_languages.restype = CStringArray
        array = lib.ffi_nlu_ontology_supported_languages()
        _ALL_LANGUAGES = set(
            array.data[i].decode("utf8") for i in range(array.size))
    return _ALL_LANGUAGES


def get_all_builtin_entities():
    """Lists the builtin entities that are supported in at least one
    language"""
    global _ALL_BUILTIN_ENTITIES
    if _ALL_BUILTIN_ENTITIES is None:
        lib.ffi_nlu_ontology_all_builtin_entities.restype = CStringArray
        array = lib.ffi_nlu_ontology_all_builtin_entities()
        _ALL_BUILTIN_ENTITIES = set(
            array.data[i].decode("utf8") for i in range(array.size))
    return _ALL_BUILTIN_ENTITIES


def get_supported_entities(language):
    """Lists the builtin entities supported in the specified *language*

    Returns:
          list of str: the list of entity labels
    """
    global _SUPPORTED_ENTITIES

    if not isinstance(language, str):
        raise TypeError("Expected language to be of type 'str' but found: %s"
                        % type(language))

    if language not in _SUPPORTED_ENTITIES:
        with string_array_pointer(pointer(CStringArray())) as ptr:
            exit_code = lib.ffi_nlu_ontology_supported_builtin_entities(
                language.encode("utf8"), byref(ptr))
            if exit_code:
                raise ValueError("Something wrong happened while retrieving "
                                 "supported entities. See stderr.")
            array = ptr.contents
            _SUPPORTED_ENTITIES[language] = set(
                array.data[i].decode("utf8") for i in range(array.size))
    return _SUPPORTED_ENTITIES[language]


class BuiltinEntityParser(object):
    """Extract builtin entities

    Args:
        language (str): Language (ISO code) of the builtin entity parser
    """

    def __init__(self, language):
        if not isinstance(language, str):
            raise TypeError("Expected language to be of type 'str' but found:"
                            " %s" % type(language))
        self.language = language
        self._parser = pointer(c_void_p())
        exit_code = lib.ffi_nlu_ontology_create_builtin_entity_parser(
            byref(self._parser), language.encode("utf8"))
        if exit_code:
            raise ImportError("Something wrong happened while creating the "
                              "intent parser. See stderr.")

    def __del__(self):
        if hasattr(self, '_parser'):
            lib.ffi_nlu_ontology_destroy_builtin_entity_parser(self._parser)

    def parse(self, text, scope=None):
        """Extract builtin entities from *text*

        Args:
            text (str): Input
            scope (list of str, optional): List of builtin entity labels. If
                defined, the parser will extract entities using the provided
                scope instead of the entire scope of all available entities.
                This allows to look for specifics builtin entity kinds.

        Returns:
            list of dict: The list of extracted entities
        """
        if not isinstance(text, str):
            raise TypeError("Expected language to be of type 'str' but found: "
                            "%s" % type(text))
        if scope is not None:
            if not all(isinstance(e, str) for e in scope):
                raise TypeError(
                    "Expected scope to contain objects of type 'str'")
            scope = [e.encode("utf8") for e in scope]
            arr = CStringArray()
            arr.size = c_int(len(scope))
            arr.data = (c_char_p * len(scope))(*scope)
            scope = byref(arr)

        with string_pointer(c_char_p()) as ptr:
            exit_code = lib.ffi_nlu_ontology_extract_entities_json(
                self._parser, text.encode("utf8"), scope, byref(ptr))
            if exit_code:
                raise ValueError("Something wrong happened while extracting "
                                 "builtin entities. See stderr.")
            result = string_at(ptr)
            return json.loads(result.decode("utf8"))
