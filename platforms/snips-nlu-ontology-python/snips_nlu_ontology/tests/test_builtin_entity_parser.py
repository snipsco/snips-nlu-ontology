from __future__ import unicode_literals

import os
import unittest

from snips_nlu_ontology import BuiltinEntityParser, get_all_languages


class TestBuiltinEntityParser(unittest.TestCase):
    def test_should_parse_without_scope(self):
        # Given
        parser = BuiltinEntityParser("en")

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
        parser = BuiltinEntityParser("en")
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
        resource_path = os.path.join(os.path.dirname(__file__), "resources",
                                     "musicArtist.json")
        gazetteer_entity_configurations = [
            {
                "builtin_entity_name": "snips/musicArtist",
                "resource_path": resource_path,
                "parser_threshold": 1.0
            }
        ]
        parser = BuiltinEntityParser("en", gazetteer_entity_configurations)
        scope = ["snips/musicArtist"]

        # When
        res = parser.parse("I want to list to calvin Harris please!", scope)

        # Then
        expected_result = [
            {
                "entity": {
                    "kind": "MusicArtist",
                    "value": "Calvin Harris"
                },
                "entity_kind": "snips/musicArtist",
                "range": {"end": 31, "start": 18},
                "value": "calvin Harris"
            }
        ]

        self.assertListEqual(expected_result, res)

    def test_should_parse_in_all_languages(self):
        # Given
        all_languages = get_all_languages()
        text = "1234"

        # When / Then
        for language in all_languages:
            parser = BuiltinEntityParser(language)
            parser.parse(text)

    def test_should_not_accept_bytes_as_language(self):
        with self.assertRaises(TypeError):
            BuiltinEntityParser(b"en")

    def test_should_not_accept_bytes_in_text(self):
        # Given
        parser = BuiltinEntityParser("en")
        bytes_text = b"Raise to sixty"

        # When/Then
        with self.assertRaises(TypeError):
            parser.parse(bytes_text)

    def test_should_not_accept_bytes_in_scope(self):
        # Given
        scope = [b"snips/number", b"snips/datetime"]
        parser = BuiltinEntityParser("en")

        # When/Then
        with self.assertRaises(TypeError):
            parser.parse("Raise to sixty", scope)
