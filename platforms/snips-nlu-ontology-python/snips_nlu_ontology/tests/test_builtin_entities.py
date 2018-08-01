import unittest
from builtins import str

from snips_nlu_ontology.builtin_entities import (
    get_all_builtin_entities, get_all_gazetteer_entities,
    get_all_grammar_entities, get_all_languages, get_builtin_entity_examples,
    get_builtin_entity_shortname, get_ontology_version, get_supported_entities,
    get_supported_gazetteer_entities, get_supported_grammar_entities)


class TestBuiltinEntities(unittest.TestCase):
    def test_should_get_all_languages(self):
        # When
        all_languages = get_all_languages()

        # Then
        self.assertIn(u"en", all_languages)
        self.assertIn(u"fr", all_languages)
        for language in all_languages:
            self.assertIsInstance(language, str)

    def test_should_get_builtin_entity_shortname(self):
        # Given
        entity_name = u"snips/amountOfMoney"

        # When
        short_name = get_builtin_entity_shortname(entity_name)

        # Then
        self.assertEqual(u"AmountOfMoney", short_name)

    def test_should_get_all_builtin_entities(self):
        # When
        all_builtins = get_all_builtin_entities()

        # Then
        self.assertIn(u"snips/number", all_builtins)
        self.assertIn(u"snips/musicArtist", all_builtins)
        for builtin in all_builtins:
            self.assertIsInstance(builtin, str)

    def test_should_get_all_grammar_entities(self):
        # When
        all_grammar_entities = get_all_grammar_entities()

        # Then
        self.assertIn(u"snips/number", all_grammar_entities)
        self.assertNotIn(u"snips/musicArtist", all_grammar_entities)
        for builtin in all_grammar_entities:
            self.assertIsInstance(builtin, str)

    def test_should_get_all_gazetteer_entities(self):
        # When
        all_gazetteer_entities = get_all_gazetteer_entities()

        # Then
        self.assertNotIn(u"snips/number", all_gazetteer_entities)
        self.assertIn(u"snips/musicArtist", all_gazetteer_entities)
        for builtin in all_gazetteer_entities:
            self.assertIsInstance(builtin, str)

    def test_should_get_supported_builtin_entities(self):
        # When
        supported_entities = get_supported_entities(u"en")

        # Then
        self.assertIn(u"snips/number", supported_entities)
        self.assertIn(u"snips/datetime", supported_entities)
        for builtin in supported_entities:
            self.assertIsInstance(builtin, str)

    def test_should_get_supported_gazetteer_entities(self):
        # When
        supported_entities = get_supported_gazetteer_entities(u"fr")

        # Then
        self.assertIn(u"snips/musicArtist", supported_entities)
        self.assertIn(u"snips/musicAlbum", supported_entities)
        self.assertNotIn(u"snips/number", supported_entities)
        for builtin in supported_entities:
            self.assertIsInstance(builtin, str)

    def test_should_get_supported_grammar_entities(self):
        # When
        supported_entities = get_supported_grammar_entities(u"en")

        # Then
        self.assertIn(u"snips/number", supported_entities)
        self.assertIn(u"snips/datetime", supported_entities)
        for builtin in supported_entities:
            self.assertIsInstance(builtin, str)

    def test_should_get_ontology_version(self):
        get_ontology_version()

    def test_should_get_builtin_entity_examples(self):
        for language in get_all_languages():
            for builtin_entity in get_supported_entities(language):
                examples = get_builtin_entity_examples(builtin_entity,
                                                       language)
                self.assertGreaterEqual(len(examples), 1)
