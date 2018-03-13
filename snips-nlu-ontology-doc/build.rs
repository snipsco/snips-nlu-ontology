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

    readme.push_str("Ontology of the Snips NLU library API which describes supported languages and builtin entities.\n");
    readme.push_str("Please refer to `this page <snips-nlu-ontology-ffi/platforms/snips-nlu-ontology-python>`_ for the python wrapper.\n");
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
    table.set_titles(row!["Entity", "Identifier", "Supported languages"]);

    let mut all_entities = ::BuiltinEntityKind::all().iter().collect::<Vec<_>>();
    all_entities.sort_by(|a, b| a.identifier().cmp(b.identifier()));

    for entity in all_entities.clone() {
        let supported_languages: String = entity
            .supported_languages()
            .iter()
            .map(|l| format!("| {}\n", l.full_name()))
            .collect();
        table.add_row(row![
            entity.to_string(),
            entity.identifier(),
            supported_languages
        ]);
    }
    readme.push_str(&*table.to_string());
    readme.push_str("\n");

    readme.push_str("Results Examples\n");
    readme.push_str("----------------\n");

    readme.push_str("\n");

    readme.push_str("The following sections provide results examples for each builtin entity.\n");

    readme.push_str("\n");

    for entity in all_entities {
        add_builtin_entity_results_examples(readme, *entity);
    }
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
    readme.push_str(&*entity.result_description().unwrap().replace("\n", "\n   "));
    readme.push_str("\n\n");
}
