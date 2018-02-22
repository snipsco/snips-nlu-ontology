Snips NLU Ontology
==================

.. image:: https://travis-ci.org/snipsco/snips-nlu-ontology.svg?branch=master
   :target: https://travis-ci.org/snipsco/snips-nlu-ontology

Ontology of the Snips NLU library API which describes supported languages and builtin entities

Ontology version: 0.6.0

Supported languages
-------------------

+----------+------------+
| Language | Identifier |
+==========+============+
| German   | de         |
+----------+------------+
| English  | en         |
+----------+------------+
| Spanish  | es         |
+----------+------------+
| French   | fr         |
+----------+------------+
| Korean   | ko         |
+----------+------------+

Supported builtin entities
--------------------------

+---------------+---------------------+---------------------+--------------------------------------------+
| Entity        | Identifier          | Supported languages | Results Examples                           |
+===============+=====================+=====================+============================================+
| AmountOfMoney | snips/amountOfMoney | English             | [                                          |
|               |                     | French              |   {                                        |
|               |                     | German              |     "kind": "AmountOfMoney",               |
|               |                     | Spanish             |     "value": 10.05,                        |
|               |                     | Korean              |     "precision": "Approximate",            |
|               |                     |                     |     "unit": "€"                            |
|               |                     |                     |   }                                        |
|               |                     |                     | ]                                          |
+---------------+---------------------+---------------------+--------------------------------------------+
| Time          | snips/datetime      | English             | [                                          |
|               |                     | Spanish             |   {                                        |
|               |                     | French              |     "kind": "InstantTime",                 |
|               |                     | Korean              |     "value": "2017-06-13 18:00:00 +02:00", |
|               |                     | German              |     "grain": "Hour",                       |
|               |                     |                     |     "precision": "Exact"                   |
|               |                     |                     |   },                                       |
|               |                     |                     |   {                                        |
|               |                     |                     |     "kind": "TimeInterval",                |
|               |                     |                     |     "from": "2017-06-07 18:00:00 +02:00",  |
|               |                     |                     |     "to": "2017-06-08 00:00:00 +02:00"     |
|               |                     |                     |   }                                        |
|               |                     |                     | ]                                          |
+---------------+---------------------+---------------------+--------------------------------------------+
| Duration      | snips/duration      | English             | [                                          |
|               |                     | Spanish             |   {                                        |
|               |                     | French              |     "kind": "Duration",                    |
|               |                     | Korean              |     "years": 0,                            |
|               |                     | German              |     "quarters": 0,                         |
|               |                     |                     |     "months": 3,                           |
|               |                     |                     |     "weeks": 0,                            |
|               |                     |                     |     "days": 0,                             |
|               |                     |                     |     "hours": 0,                            |
|               |                     |                     |     "minutes": 0,                          |
|               |                     |                     |     "seconds": 0,                          |
|               |                     |                     |     "precision": "Exact"                   |
|               |                     |                     |   }                                        |
|               |                     |                     | ]                                          |
+---------------+---------------------+---------------------+--------------------------------------------+
| Number        | snips/number        | English             | [                                          |
|               |                     | Spanish             |   {                                        |
|               |                     | French              |     "kind": "Number",                      |
|               |                     | Korean              |     "value": 42.0                          |
|               |                     | German              |   }                                        |
|               |                     |                     | ]                                          |
+---------------+---------------------+---------------------+--------------------------------------------+
| Ordinal       | snips/ordinal       | English             | [                                          |
|               |                     | Spanish             |   {                                        |
|               |                     | French              |     "kind": "Ordinal",                     |
|               |                     | Korean              |     "value": 2                             |
|               |                     | German              |   }                                        |
|               |                     |                     | ]                                          |
+---------------+---------------------+---------------------+--------------------------------------------+
| Percentage    | snips/percentage    | English             | [                                          |
|               |                     | Spanish             |   {                                        |
|               |                     | French              |     "kind": "Percentage",                  |
|               |                     | German              |     "value": 20.0                          |
|               |                     |                     |   }                                        |
|               |                     |                     | ]                                          |
+---------------+---------------------+---------------------+--------------------------------------------+
| Temperature   | snips/temperature   | English             | [                                          |
|               |                     | Spanish             |   {                                        |
|               |                     | French              |     "kind": "Temperature",                 |
|               |                     | Korean              |     "value": 23.0,                         |
|               |                     | German              |     "unit": "celsius"                      |
|               |                     |                     |   },                                       |
|               |                     |                     |   {                                        |
|               |                     |                     |     "kind": "Temperature",                 |
|               |                     |                     |     "value": 60.0,                         |
|               |                     |                     |     "unit": "fahrenheit"                   |
|               |                     |                     |   }                                        |
|               |                     |                     | ]                                          |
+---------------+---------------------+---------------------+--------------------------------------------+

