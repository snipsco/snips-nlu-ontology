import unittest

from builtins import str

from snips_nlu_ontology_rust.builtin_entities import (
    get_all_languages, get_all_builtin_entities)


class TestBuiltinEntities(unittest.TestCase):
    def test_should_get_all_languages(self):
        # When
        all_languages = get_all_languages()

        # Then
        self.assertIn(u"en", all_languages)
        self.assertIn(u"fr", all_languages)
        for language in all_languages:
            self.assertIsInstance(language, str)

    def test_should_get_all_builtin_entities(self):
        # When
        all_builtins = get_all_builtin_entities()

        # Then
        self.assertIn(u"snips/number", all_builtins)
        self.assertIn(u"snips/datetime", all_builtins)
        for builtin in all_builtins:
            self.assertIsInstance(builtin, str)
