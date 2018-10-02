#[macro_use]
extern crate prettytable;
extern crate snips_nlu_ontology;

use std::fs::File;
use std::io::prelude::*;

use prettytable::Table;
use snips_nlu_ontology::*;

fn main() {
    let mut readme = String::new();
    add_header(&mut readme);
    add_supported_languages(&mut readme);
    add_supported_builtin_entities(&mut readme);
    add_grammar_entity_documentation(&mut readme);
    add_gazetteer_entity_documentation(&mut readme);
    add_builtin_entities_examples(&mut readme);
    add_footer(&mut readme);

    let mut file = File::create("../README.rst").unwrap();
    file.write_all(readme.as_bytes()).unwrap();
}

fn add_header(readme: &mut String) {
    readme.push_str("Snips NLU Ontology\n");
    readme.push_str("==================\n");
    readme.push_str("\n");

    readme.push_str(
        ".. image:: https://travis-ci.org/snipsco/snips-nlu-ontology.svg?branch=develop\n",
    );
    readme.push_str("   :target: https://travis-ci.org/snipsco/snips-nlu-ontology\n");
    readme.push_str("\n");
    readme.push_str(
        ".. image:: https://ci.appveyor.com/api/projects/status/github/snipsco/snips-nlu-ontology?branch=develop&svg=true\n",
    );
    readme.push_str("   :target: https://ci.appveyor.com/project/snipsco/snips-nlu-ontology\n");
    readme.push_str("\n");

    readme.push_str("Ontology of the Snips NLU library API which describes supported languages and builtin entities.\n");
    readme.push_str("Please refer to `this page <platforms/snips-nlu-ontology-python>`_ for the python wrapper.\n");
    readme.push_str("\n");

    readme.push_str(&*format!("Ontology version: {}\n", ::ONTOLOGY_VERSION));
    readme.push_str("\n");
}

fn add_supported_languages(readme: &mut String) {
    readme.push_str("Supported languages\n");
    readme.push_str("-------------------\n");
    readme.push_str("\n");
    let mut table = Table::new();
    table.set_format(*prettytable::format::consts::FORMAT_DEFAULT);
    table.set_titles(row!["Language", "Identifier"]);
    for language in ::Language::all().iter() {
        table.add_row(row![language.full_name(), language.to_string()]);
    }
    readme.push_str(&*table.to_string());
    readme.push_str("\n");
}

fn add_supported_builtin_entities(readme: &mut String) {
    readme.push_str("Supported builtin entities\n");
    readme.push_str("--------------------------\n");
    readme.push_str("\n");
    let mut table = Table::new();
    table.set_format(*prettytable::format::consts::FORMAT_DEFAULT);
    table.set_titles(row!["Entity", "Identifier", "Category", "Supported languages"]);

    let mut all_entities = ::BuiltinEntityKind::all().iter().collect::<Vec<_>>();
    all_entities.sort_by(|a, b| a.identifier().cmp(b.identifier()));

    for entity in all_entities.clone() {
        let supported_languages: String = entity
            .supported_languages()
            .iter()
            .map(|l| format!("| {}\n", l.full_name()))
            .collect();
        let category = BuiltinGazetteerEntityKind::from_identifier(entity.identifier())
            .ok()
            .map(|_| "`Gazetteer Entity`_".to_string())
            .unwrap_or_else(|| "`Grammar Entity`_".to_string());
        table.add_row(row![
            entity.to_string(),
            entity.identifier(),
            category,
            supported_languages
        ]);
    }
    readme.push_str(&*table.to_string());
    readme.push_str("\n");
}

fn add_builtin_entities_examples(readme: &mut String) {
    let mut all_entities = ::BuiltinEntityKind::all().iter().collect::<Vec<_>>();
    all_entities.sort_by(|a, b| a.identifier().cmp(b.identifier()));
    readme.push_str("Results Examples\n");
    readme.push_str("----------------\n");

    readme.push_str("\n");

    readme.push_str("The following sections provide results examples for each builtin entity.\n");

    readme.push_str("\n");

    for entity in all_entities {
        add_builtin_entity_results_examples(readme, *entity);
    }
}

fn add_grammar_entity_documentation(readme: &mut String) {
    readme.push_str("Grammar Entity\n");
    readme.push_str("--------------\n");

    readme.push_str("\n");

    readme.push_str(
        "Grammar entities, in the context of Snips NLU, correspond to entities which contain \
        significant `compositionality`_. The semantic meaning of such an entity is determined by \
        the meanings of its constituent expressions and the rules used to combine them. Modern \
        semantic parsers for these entities are often based on defining a formal grammar. In the \
        case of Snips NLU, the parser used to handle these entities is `Rustling`_, a Rust \
        adaptation of Facebook's `duckling`_.\n"
    );

    readme.push_str("\n");

}

fn add_gazetteer_entity_documentation(readme: &mut String) {
    readme.push_str("Gazetteer Entity\n");
    readme.push_str("----------------\n");

    readme.push_str("\n");

    readme.push_str(
        "Gazetteer entities correspond to all the builtin entities which do not contain any \
        semantical structure, as opposed to the grammar entities. For such entities, a \
        `gazetteer entity parser`_ is used to perform the parsing.\n"
    );

    readme.push_str("\n");

}

fn add_builtin_entity_results_examples(readme: &mut String, entity: BuiltinEntityKind) {
    let mut entity_title = Table::new();
    entity_title.set_format(*prettytable::format::consts::FORMAT_NO_COLSEP);
    entity_title.add_row(row![entity.to_string()]);
    let cleaned_title = entity_title
        .to_string()
        .replace(" ", "")
        .replace("--\n", "\n");
    readme.push_str(&*cleaned_title);
    readme.push_str("\n");
    readme.push_str(".. code-block:: json\n");
    readme.push_str("\n   ");
    readme.push_str(&*entity.result_description().replace("\n", "\n   "));
    readme.push_str("\n\n");
}

fn add_footer(readme: &mut String) {
    readme.push_str(".. _compositionality: https://en.wikipedia.org/wiki/Principle_of_compositionality\n");
    readme.push_str(".. _Rustling: https://github.com/snipsco/rustling-ontology\n");
    readme.push_str(".. _duckling: https://github.com/facebook/duckling\n");
    readme.push_str(".. _gazetteer entity parser: https://github.com/snipsco/gazetteer-entity-parser\n");
}
