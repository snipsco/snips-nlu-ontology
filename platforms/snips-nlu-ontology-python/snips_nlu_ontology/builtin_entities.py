# coding=utf-8
from __future__ import (absolute_import, division, print_function,
                        unicode_literals)

from _ctypes import byref, pointer
from builtins import range, str
from ctypes import c_char_p, string_at

from snips_nlu_ontology.utils import (CStringArray, check_ffi_error, lib,
                                      string_array_pointer, string_pointer)

_ALL_LANGUAGES = None
_SUPPORTED_ENTITIES = dict()
_SUPPORTED_GAZETTEER_ENTITIES = dict()
_SUPPORTED_GRAMMAR_ENTITIES = dict()
_ENTITIES_EXAMPLES = dict()
_ALL_BUILTIN_ENTITIES = None
_ALL_GAZETTEER_ENTITIES = None
_ALL_GRAMMAR_ENTITIES = None
_BUILTIN_ENTITIES_SHORTNAMES = dict()
_ONTOLOGY_VERSION = None


def get_ontology_version():
    """Get the version of the ontology"""
    global _ONTOLOGY_VERSION
    if _ONTOLOGY_VERSION is None:
        lib.snips_nlu_ontology_version.restype = c_char_p
        _ONTOLOGY_VERSION = lib.snips_nlu_ontology_version().decode("utf8")
    return _ONTOLOGY_VERSION


def get_all_languages():
    """Lists all the supported languages"""
    global _ALL_LANGUAGES
    if _ALL_LANGUAGES is None:
        lib.snips_nlu_ontology_supported_languages.restype = CStringArray
        array = lib.snips_nlu_ontology_supported_languages()
        _ALL_LANGUAGES = set(
            array.data[i].decode("utf8") for i in range(array.size))
    return _ALL_LANGUAGES


def get_all_builtin_entities():
    """Lists the builtin entities that are supported in at least one
    language"""
    global _ALL_BUILTIN_ENTITIES
    if _ALL_BUILTIN_ENTITIES is None:
        lib.snips_nlu_ontology_all_builtin_entities.restype = CStringArray
        array = lib.snips_nlu_ontology_all_builtin_entities()
        _ALL_BUILTIN_ENTITIES = set(
            array.data[i].decode("utf8") for i in range(array.size))
    return _ALL_BUILTIN_ENTITIES


def get_all_gazetteer_entities():
    """Lists the gazetteer entities that are supported in at least one
    language"""
    global _ALL_GAZETTEER_ENTITIES
    if _ALL_GAZETTEER_ENTITIES is None:
        lib.snips_nlu_ontology_all_gazetteer_entities.restype = CStringArray
        array = lib.snips_nlu_ontology_all_gazetteer_entities()
        _ALL_GAZETTEER_ENTITIES = set(
            array.data[i].decode("utf8") for i in range(array.size))
    return _ALL_GAZETTEER_ENTITIES


def get_all_grammar_entities():
    """Lists the grammar entities that are supported in at least one
    language"""
    global _ALL_GRAMMAR_ENTITIES
    if _ALL_GRAMMAR_ENTITIES is None:
        lib.snips_nlu_ontology_all_grammar_entities.restype = CStringArray
        array = lib.snips_nlu_ontology_all_grammar_entities()
        _ALL_GRAMMAR_ENTITIES = set(
            array.data[i].decode("utf8") for i in range(array.size))
    return _ALL_GRAMMAR_ENTITIES


def get_builtin_entity_shortname(entity):
    """Get the short name of the entity

    Examples:

    >>> get_builtin_entity_shortname(u"snips/amountOfMoney")
    'AmountOfMoney'
    """
    global _BUILTIN_ENTITIES_SHORTNAMES
    if entity not in _BUILTIN_ENTITIES_SHORTNAMES:
        with string_pointer(c_char_p()) as ptr:
            exit_code = lib.snips_nlu_ontology_entity_shortname(
                entity.encode("utf8"), byref(ptr))
            check_ffi_error(exit_code, "Something went wrong when retrieving "
                                       "builtin entity shortname")
            result = string_at(ptr)
            _BUILTIN_ENTITIES_SHORTNAMES[entity] = result.decode("utf8")
    return _BUILTIN_ENTITIES_SHORTNAMES[entity]


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
            exit_code = lib.snips_nlu_ontology_supported_builtin_entities(
                language.encode("utf8"), byref(ptr))
            check_ffi_error(exit_code, "Something went wrong when retrieving "
                                       "supported entities")
            array = ptr.contents
            _SUPPORTED_ENTITIES[language] = set(
                array.data[i].decode("utf8") for i in range(array.size))
    return _SUPPORTED_ENTITIES[language]


def get_supported_gazetteer_entities(language):
    """Lists the gazetteer entities supported in the specified *language*

    Returns:
          list of str: the list of entity labels
    """
    global _SUPPORTED_GAZETTEER_ENTITIES

    if not isinstance(language, str):
        raise TypeError("Expected language to be of type 'str' but found: %s"
                        % type(language))

    if language not in _SUPPORTED_GAZETTEER_ENTITIES:
        with string_array_pointer(pointer(CStringArray())) as ptr:
            exit_code = \
                lib.snips_nlu_ontology_supported_builtin_gazetteer_entities(
                    language.encode("utf8"), byref(ptr))
            check_ffi_error(exit_code, "Something went wrong when retrieving "
                                       "supported gazetteer entities")
            array = ptr.contents
            _SUPPORTED_GAZETTEER_ENTITIES[language] = set(
                array.data[i].decode("utf8") for i in range(array.size))
    return _SUPPORTED_GAZETTEER_ENTITIES[language]


def get_supported_grammar_entities(language):
    """Lists the grammar entities supported in the specified *language*

    Returns:
          list of str: the list of entity labels
    """
    global _SUPPORTED_GRAMMAR_ENTITIES

    if not isinstance(language, str):
        raise TypeError("Expected language to be of type 'str' but found: %s"
                        % type(language))

    if language not in _SUPPORTED_GRAMMAR_ENTITIES:
        with string_array_pointer(pointer(CStringArray())) as ptr:
            exit_code = lib.snips_nlu_ontology_supported_grammar_entities(
                language.encode("utf8"), byref(ptr))
            check_ffi_error(exit_code, "Something went wrong when retrieving "
                                       "supported grammar entities")
            array = ptr.contents
            _SUPPORTED_GRAMMAR_ENTITIES[language] = set(
                array.data[i].decode("utf8") for i in range(array.size))
    return _SUPPORTED_GRAMMAR_ENTITIES[language]


def get_builtin_entity_examples(builtin_entity_kind, language):
    """Provides some examples of the builtin entity in the specified language
    """
    global _ENTITIES_EXAMPLES

    if not isinstance(builtin_entity_kind, str):
        raise TypeError("Expected `builtin_entity_kind` to be of type 'str' "
                        "but found: %s" % type(builtin_entity_kind))
    if not isinstance(language, str):
        raise TypeError("Expected `language` to be of type 'str' but found: %s"
                        % type(language))

    if builtin_entity_kind not in _ENTITIES_EXAMPLES:
        _ENTITIES_EXAMPLES[builtin_entity_kind] = dict()

    if language not in _ENTITIES_EXAMPLES[builtin_entity_kind]:
        with string_array_pointer(pointer(CStringArray())) as ptr:
            exit_code = lib.snips_nlu_ontology_builtin_entity_examples(
                builtin_entity_kind.encode("utf8"),
                language.encode("utf8"), byref(ptr))
            check_ffi_error(exit_code, "Something went wrong when retrieving "
                                       "builtin entity examples")
            array = ptr.contents
            _ENTITIES_EXAMPLES[builtin_entity_kind][language] = list(
                array.data[i].decode("utf8") for i in range(array.size))
    return _ENTITIES_EXAMPLES[builtin_entity_kind][language]
