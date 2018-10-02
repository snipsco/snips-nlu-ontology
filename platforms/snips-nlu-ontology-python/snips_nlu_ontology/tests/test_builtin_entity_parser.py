from __future__ import unicode_literals

import unittest

from snips_nlu_ontology import BuiltinEntityParser, get_all_languages
from snips_nlu_ontology.tests.test_utils import ROOT_DIR
from snips_nlu_ontology.utils import temp_dir

BUILTIN_PARSER_PATH = ROOT_DIR / "data" / "tests" / "builtin_entity_parser"
BUILTIN_PARSER_NO_GAZETTEER_PATH = ROOT_DIR / "data" / "tests" / \
                                   "builtin_entity_parser_no_gazetteer"


class TestBuiltinEntityParser(unittest.TestCase):
    def test_should_parse_without_scope(self):
        # Given
        parser = BuiltinEntityParser.build("en")

        # When
        res = parser.parse("Raise to sixty two degrees celsius")

        # Then
        expected_result = [
            {
                "entity": {
                    "kind": "Temperature",
                    "unit": "celsius",
                    "value": 62.0
                },
                "entity_kind": "snips/temperature",
                "range": {"end": 34, "start": 9},
                "value": "sixty two degrees celsius"
            }
        ]

        self.assertListEqual(expected_result, res)

    def test_should_parse_with_scope(self):
        # Given
        parser = BuiltinEntityParser.build("en")
        scope = ["snips/duration", "snips/temperature"]

        # When
        res = parser.parse("Raise to sixty two", scope)

        # Then
        expected_result = [
            {
                "entity": {
                    "kind": "Temperature",
                    "unit": None,
                    "value": 62.0
                },
                "entity_kind": "snips/temperature",
                "range": {"end": 18, "start": 9},
                "value": "sixty two"
            }
        ]

        self.assertListEqual(expected_result, res)

    def test_should_parse_with_gazetteer_entity(self):
        # Given
        gazetteer_parser_path = ROOT_DIR / "data" / "tests" / \
                                "builtin_gazetteer_parser"
        parser = BuiltinEntityParser.build("en", gazetteer_parser_path)
        scope = ["snips/musicArtist"]

        # When
        res = parser.parse("I want to listen to the stones please!", scope)

        # Then
        expected_result = [
            {
                "entity": {
                    "kind": "MusicArtist",
                    "value": "The Rolling Stones"
                },
                "entity_kind": "snips/musicArtist",
                "range": {"end": 30, "start": 20},
                "value": "the stones"
            }
        ]

        self.assertListEqual(expected_result, res)

    def test_should_parse_in_all_languages(self):
        # Given
        all_languages = get_all_languages()
        text = "1234"

        # When / Then
        for language in all_languages:
            parser = BuiltinEntityParser.build(language)
            parser.parse(text)

    def test_should_persist_parser(self):
        # Given
        parser = BuiltinEntityParser.build("en")

        # When
        with temp_dir() as tmpdir:
            persisted_path = str(tmpdir / "persisted_builtin_parser")
            parser.persist(persisted_path)
            loaded_parser = BuiltinEntityParser.from_path(persisted_path)
        res = loaded_parser.parse("Raise the temperature to 9 degrees", None)

        # Then
        expected_result = [
            {
                "value": "9 degrees",
                "entity": {
                    "kind": "Temperature",
                    "unit": "degree",
                    "value": 9.0
                },
                "range": {"start": 25, "end": 34},
                "entity_kind": "snips/temperature"
            }
        ]
        self.assertListEqual(expected_result, res)

    def test_should_load_parser_from_path(self):
        # Given
        parser = BuiltinEntityParser.from_path(
            BUILTIN_PARSER_NO_GAZETTEER_PATH)

        # When
        res = parser.parse("Raise the temperature to 9 degrees", None)

        # Then
        expected_result = [
            {
                "value": "9 degrees",
                "entity": {
                    "kind": "Temperature",
                    "unit": "degree",
                    "value": 9.0
                },
                "range": {"start": 25, "end": 34},
                "entity_kind": "snips/temperature"
            }
        ]

        self.assertListEqual(expected_result, res)

    def test_should_persist_parser_with_gazetteer_entities(self):
        # Given
        parser = BuiltinEntityParser.from_path(BUILTIN_PARSER_PATH)

        # When
        with temp_dir() as tmpdir:
            persisted_path = str(tmpdir / "persisted_builtin_parser")
            parser.persist(persisted_path)
            loaded_parser = BuiltinEntityParser.from_path(persisted_path)
        res = loaded_parser.parse("I want to listen to the stones", None)

        # Then
        expected_result = [
            {
                "value": "the stones",
                "entity": {
                    "kind": "MusicArtist",
                    "value": "The Rolling Stones"
                },
                "range": {"start": 20, "end": 30},
                "entity_kind": "snips/musicArtist"
            }
        ]
        self.assertListEqual(expected_result, res)

    def test_should_load_parser_with_gazetteer_entities_from_path(self):
        # Given
        parser = BuiltinEntityParser.from_path(BUILTIN_PARSER_PATH)

        # When
        res = parser.parse("I want to listen to the stones", None)

        # Then
        expected_result = [
            {
                "value": "the stones",
                "entity": {
                    "kind": "MusicArtist",
                    "value": "The Rolling Stones"
                },
                "range": {"start": 20, "end": 30},
                "entity_kind": "snips/musicArtist"
            }
        ]
        self.assertListEqual(expected_result, res)

    def test_should_not_accept_bytes_as_language(self):
        with self.assertRaises(TypeError):
            BuiltinEntityParser.build(b"en")

    def test_should_not_accept_bytes_in_text(self):
        # Given
        parser = BuiltinEntityParser.build("en")
        bytes_text = b"Raise to sixty"

        # When/Then
        with self.assertRaises(TypeError):
            parser.parse(bytes_text)

    def test_should_not_accept_bytes_in_scope(self):
        # Given
        scope = [b"snips/number", b"snips/datetime"]
        parser = BuiltinEntityParser.build("en")

        # When/Then
        with self.assertRaises(TypeError):
            parser.parse("Raise to sixty", scope)
