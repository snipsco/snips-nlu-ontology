import json
from _ctypes import byref, pointer
from ctypes import c_char_p, c_int, c_void_p, string_at
from pathlib import Path

from snips_nlu_ontology.utils import CStringArray, lib, string_pointer


class BuiltinEntityParser(object):
    """Extract builtin entities

    Args:
        language (str): Language (ISO code) of the builtin entity parser
        gazetteer_entity_parser_path (str, opt): Path to the gazetteer entity
            parser.
    """

    def __init__(self, language, gazetteer_entity_parser_path=None):
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
        if exit_code:
            raise ImportError("Something went wrong while creating the "
                              "builtin entity parser. See stderr.")
        self._parser = parser

    def __del__(self):
        if lib is not None and hasattr(self, '_parser'):
            lib.snips_nlu_ontology_destroy_builtin_entity_parser(self._parser)

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
            if exit_code:
                raise ValueError("Something went wrong while extracting "
                                 "builtin entities. See stderr.")
            result = string_at(ptr)
            return json.loads(result.decode("utf8"))
