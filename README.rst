Snips NLU Ontology
==================

.. image:: https://travis-ci.org/snipsco/snips-nlu-ontology.svg?branch=develop
   :target: https://travis-ci.org/snipsco/snips-nlu-ontology

Ontology of the Snips NLU library API which describes supported languages and builtin entities.

Important Note
--------------

The code related to builtin entity parsing has been moved to the `snips-nlu-parsers`_ repository.

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
| Italian  | it         |
+----------+------------+
| Japanese | ja         |
+----------+------------+
| Korean   | ko         |
+----------+------------+

Supported builtin entities
--------------------------

+---------------+---------------------+---------------------+---------------------+
| Entity        | Identifier          | Category            | Supported languages |
+===============+=====================+=====================+=====================+
| AmountOfMoney | snips/amountOfMoney | `Grammar Entity`_   | | German            |
|               |                     |                     | | English           |
|               |                     |                     | | Spanish           |
|               |                     |                     | | French            |
|               |                     |                     | | Italian           |
|               |                     |                     | | Japanese          |
|               |                     |                     | | Korean            |
+---------------+---------------------+---------------------+---------------------+
| Time          | snips/datetime      | `Grammar Entity`_   | | German            |
|               |                     |                     | | English           |
|               |                     |                     | | Spanish           |
|               |                     |                     | | French            |
|               |                     |                     | | Italian           |
|               |                     |                     | | Japanese          |
|               |                     |                     | | Korean            |
+---------------+---------------------+---------------------+---------------------+
| Duration      | snips/duration      | `Grammar Entity`_   | | German            |
|               |                     |                     | | English           |
|               |                     |                     | | Spanish           |
|               |                     |                     | | French            |
|               |                     |                     | | Italian           |
|               |                     |                     | | Japanese          |
|               |                     |                     | | Korean            |
+---------------+---------------------+---------------------+---------------------+
| MusicAlbum    | snips/musicAlbum    | `Gazetteer Entity`_ | | English           |
|               |                     |                     | | French            |
+---------------+---------------------+---------------------+---------------------+
| MusicArtist   | snips/musicArtist   | `Gazetteer Entity`_ | | English           |
|               |                     |                     | | French            |
+---------------+---------------------+---------------------+---------------------+
| MusicTrack    | snips/musicTrack    | `Gazetteer Entity`_ | | English           |
|               |                     |                     | | French            |
+---------------+---------------------+---------------------+---------------------+
| Number        | snips/number        | `Grammar Entity`_   | | German            |
|               |                     |                     | | English           |
|               |                     |                     | | Spanish           |
|               |                     |                     | | French            |
|               |                     |                     | | Italian           |
|               |                     |                     | | Japanese          |
|               |                     |                     | | Korean            |
+---------------+---------------------+---------------------+---------------------+
| Ordinal       | snips/ordinal       | `Grammar Entity`_   | | German            |
|               |                     |                     | | English           |
|               |                     |                     | | Spanish           |
|               |                     |                     | | French            |
|               |                     |                     | | Italian           |
|               |                     |                     | | Japanese          |
|               |                     |                     | | Korean            |
+---------------+---------------------+---------------------+---------------------+
| Percentage    | snips/percentage    | `Grammar Entity`_   | | German            |
|               |                     |                     | | English           |
|               |                     |                     | | Spanish           |
|               |                     |                     | | French            |
|               |                     |                     | | Italian           |
|               |                     |                     | | Japanese          |
+---------------+---------------------+---------------------+---------------------+
| Temperature   | snips/temperature   | `Grammar Entity`_   | | German            |
|               |                     |                     | | English           |
|               |                     |                     | | Spanish           |
|               |                     |                     | | French            |
|               |                     |                     | | Italian           |
|               |                     |                     | | Japanese          |
|               |                     |                     | | Korean            |
+---------------+---------------------+---------------------+---------------------+

Grammar Entity
--------------

Grammar entities, in the context of Snips NLU, correspond to entities which contain significant `compositionality`_. The semantic meaning of such an entity is determined by the meanings of its constituent expressions and the rules used to combine them. Modern semantic parsers for these entities are often based on defining a formal grammar. In the case of Snips NLU, the parser used to handle these entities is `Rustling`_, a Rust adaptation of Facebook's `duckling`_.

Gazetteer Entity
----------------

Gazetteer entities correspond to all the builtin entities which do not contain any semantical structure, as opposed to the grammar entities. For such entities, a `gazetteer entity parser`_ is used to perform the parsing.

Results Examples
----------------

The following sections provide results examples for each builtin entity.

-------------
AmountOfMoney
-------------

.. code-block:: json

   [
     {
       "kind": "AmountOfMoney",
       "value": 10.05,
       "precision": "Approximate",
       "unit": "€"
     }
   ]

----
Time
----

.. code-block:: json

   [
     {
       "kind": "InstantTime",
       "value": "2017-06-13 18:00:00 +02:00",
       "grain": "Hour",
       "precision": "Exact"
     },
     {
       "kind": "TimeInterval",
       "from": "2017-06-07 18:00:00 +02:00",
       "to": "2017-06-08 00:00:00 +02:00"
     }
   ]

--------
Duration
--------

.. code-block:: json

   [
     {
       "kind": "Duration",
       "years": 0,
       "quarters": 0,
       "months": 3,
       "weeks": 0,
       "days": 0,
       "hours": 0,
       "minutes": 0,
       "seconds": 0,
       "precision": "Exact"
     }
   ]

----------
MusicAlbum
----------

.. code-block:: json

   [
     {
       "kind": "MusicAlbum",
       "value": "Discovery"
     }
   ]

-----------
MusicArtist
-----------

.. code-block:: json

   [
     {
       "kind": "MusicArtist",
       "value": "Daft Punk"
     }
   ]

----------
MusicTrack
----------

.. code-block:: json

   [
     {
       "kind": "MusicTrack",
       "value": "Harder Better Faster Stronger"
     }
   ]

------
Number
------

.. code-block:: json

   [
     {
       "kind": "Number",
       "value": 42.0
     }
   ]

-------
Ordinal
-------

.. code-block:: json

   [
     {
       "kind": "Ordinal",
       "value": 2
     }
   ]

----------
Percentage
----------

.. code-block:: json

   [
     {
       "kind": "Percentage",
       "value": 20.0
     }
   ]

-----------
Temperature
-----------

.. code-block:: json

   [
     {
       "kind": "Temperature",
       "value": 23.0,
       "unit": "celsius"
     },
     {
       "kind": "Temperature",
       "value": 60.0,
       "unit": "fahrenheit"
     }
   ]

.. _compositionality: https://en.wikipedia.org/wiki/Principle_of_compositionality
.. _Rustling: https://github.com/snipsco/rustling-ontology
.. _duckling: https://github.com/facebook/duckling
.. _gazetteer entity parser: https://github.com/snipsco/gazetteer-entity-parser
.. _snips-nlu-parsers: https://github.com/snipsco/snips-nlu-parsers
