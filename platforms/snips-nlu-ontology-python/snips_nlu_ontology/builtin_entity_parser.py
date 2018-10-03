import json
from _ctypes import byref, pointer
from builtins import bytes, str
from ctypes import c_char_p, c_int, c_void_p, string_at
from pathlib import Path

from snips_nlu_ontology.utils import (
    CStringArray, check_ffi_error, lib, string_pointer)


class BuiltinEntityParser(object):
    def __init__(self, parser):
        self._parser = parser

    @classmethod
    def build(cls, language, gazetteer_entity_parser_path=None):
        """Build a `BuiltinEntityParser`

        Args:
            language (str): Language identifier
            gazetteer_entity_parser_path (str, optional): Path to a gazetteer
                entity parser. If None, the builtin entity parser will only
                use grammar entities.
        """
        if isinstance(gazetteer_entity_parser_path, Path):
            gazetteer_entity_parser_path = str(gazetteer_entity_parser_path)
        if not isinstance(language, str):
            raise TypeError("Expected language to be of type 'str' but found:"
                            " %s" % type(language))
        parser_config = dict(
            language=language.upper(),
            gazetteer_parser_path=gazetteer_entity_parser_path)
        parser = pointer(c_void_p())
        json_parser_config = bytes(json.dumps(parser_config), encoding="utf8")
        exit_code = lib.snips_nlu_ontology_create_builtin_entity_parser(
            byref(parser), json_parser_config)
        check_ffi_error(exit_code, "Something went wrong while creating the "
                                   "builtin entity parser")
        return cls(parser)

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
            exit_code = lib.snips_nlu_ontology_extract_builtin_entities_json(
                self._parser, text.encode("utf8"), scope, byref(ptr))
            check_ffi_error(exit_code, "Something went wrong when extracting "
                                       "builtin entities")
            result = string_at(ptr)
            return json.loads(result.decode("utf8"))

    def persist(self, path):
        """Persist the gazetteer parser on disk at the provided path"""
        if isinstance(path, Path):
            path = str(path)
        exit_code = lib.snips_nlu_ontology_persist_builtin_entity_parser(
            self._parser, path.encode("utf8"))
        check_ffi_error(exit_code, "Something went wrong when persisting the "
                                   "builtin entity parser")

    @classmethod
    def from_path(cls, parser_path):
        """Create a :class:`GazetteerEntityParser` from a gazetteer parser
        persisted on disk
        """
        if isinstance(parser_path, Path):
            parser_path = str(parser_path)
        parser = pointer(c_void_p())
        parser_path = bytes(parser_path, encoding="utf8")
        exit_code = lib.snips_nlu_ontology_load_builtin_entity_parser(
            byref(parser), parser_path)
        check_ffi_error(exit_code, "Something went wrong when loading the "
                                   "builtin entity parser")
        return cls(parser)

    def __del__(self):
        if lib is not None and hasattr(self, '_parser'):
            lib.snips_nlu_ontology_destroy_builtin_entity_parser(self._parser)
