from __future__ import unicode_literals

import unittest

from snips_nlu_ontology_rust import BuiltinEntityParser


class TestBuiltinEntityParser(unittest.TestCase):
    def test_should_parser_without_scope(self):
        # Given
        parser = BuiltinEntityParser("en")

        # When
        res = parser.parse("what will be the weather on Apr. 1st 2018 ?")

        # Then
        expected_result = [
            {
                'value': 'on Apr. 1st 2018',
                'range': {'start': 25, 'end': 41},
                'entity': {
                    'kind': 'InstantTime',
                    'value': '2018-04-01 00:00:00 +02:00',
                    'grain': 'Day',
                    'precision': 'Exact'
                },
                'entity_kind': 'snips/datetime'
            }
        ]

        self.assertListEqual(expected_result, res)

    def test_should_parser_with_scope(self):
        # Given
        parser = BuiltinEntityParser("en")
        scope = ["snips/temperature", "snips/number"]

        # When
        res = parser.parse("Raise to sixty two", scope)

        # Then
        expected_result = [
            {
                'entity': {
                    'kind': 'Temperature',
                    'unit': None,
                    'value': 62.0
                },
                'entity_kind': 'snips/temperature',
                'range': {'end': 18, 'start': 9},
                'value': 'sixty two'
            }
        ]

        self.assertListEqual(expected_result, res)

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
