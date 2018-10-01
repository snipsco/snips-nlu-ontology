Snips NLU Ontology
==================

.. image:: https://travis-ci.org/snipsco/snips-nlu-ontology.svg?branch=develop
   :target: https://travis-ci.org/snipsco/snips-nlu-ontology

.. image:: https://ci.appveyor.com/api/projects/status/github/snipsco/snips-nlu-ontology?branch=develop&svg=true
   :target: https://ci.appveyor.com/project/snipsco/snips-nlu-ontology

Ontology of the Snips NLU library API which describes supported languages and builtin entities.
Please refer to `this page <platforms/snips-nlu-ontology-python>`_ for the python wrapper.

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
| Italian  | it         |
+----------+------------+
| Japanese | ja         |
+----------+------------+
| Korean   | ko         |
+----------+------------+

Supported builtin entities
--------------------------

+---------------+---------------------+------------------+---------------------+
| Entity        | Identifier          | Category         | Supported languages |
+===============+=====================+==================+=====================+
| AmountOfMoney | snips/amountOfMoney | Grammar Entity   | | German            |
|               |                     |                  | | English           |
|               |                     |                  | | Spanish           |
|               |                     |                  | | French            |
|               |                     |                  | | Italian           |
|               |                     |                  | | Japanese          |
|               |                     |                  | | Korean            |
+---------------+---------------------+------------------+---------------------+
| Time          | snips/datetime      | Grammar Entity   | | German            |
|               |                     |                  | | English           |
|               |                     |                  | | Spanish           |
|               |                     |                  | | French            |
|               |                     |                  | | Italian           |
|               |                     |                  | | Japanese          |
|               |                     |                  | | Korean            |
+---------------+---------------------+------------------+---------------------+
| Duration      | snips/duration      | Grammar Entity   | | German            |
|               |                     |                  | | English           |
|               |                     |                  | | Spanish           |
|               |                     |                  | | French            |
|               |                     |                  | | Italian           |
|               |                     |                  | | Japanese          |
|               |                     |                  | | Korean            |
+---------------+---------------------+------------------+---------------------+
| MusicAlbum    | snips/musicAlbum    | Gazetteer Entity | | French            |
+---------------+---------------------+------------------+---------------------+
| MusicArtist   | snips/musicArtist   | Gazetteer Entity | | French            |
+---------------+---------------------+------------------+---------------------+
| MusicTrack    | snips/musicTrack    | Gazetteer Entity | | French            |
+---------------+---------------------+------------------+---------------------+
| Number        | snips/number        | Grammar Entity   | | German            |
|               |                     |                  | | English           |
|               |                     |                  | | Spanish           |
|               |                     |                  | | French            |
|               |                     |                  | | Italian           |
|               |                     |                  | | Japanese          |
|               |                     |                  | | Korean            |
+---------------+---------------------+------------------+---------------------+
| Ordinal       | snips/ordinal       | Grammar Entity   | | German            |
|               |                     |                  | | English           |
|               |                     |                  | | Spanish           |
|               |                     |                  | | French            |
|               |                     |                  | | Italian           |
|               |                     |                  | | Japanese          |
|               |                     |                  | | Korean            |
+---------------+---------------------+------------------+---------------------+
| Percentage    | snips/percentage    | Grammar Entity   | | German            |
|               |                     |                  | | English           |
|               |                     |                  | | Spanish           |
|               |                     |                  | | French            |
|               |                     |                  | | Italian           |
|               |                     |                  | | Japanese          |
+---------------+---------------------+------------------+---------------------+
| Temperature   | snips/temperature   | Grammar Entity   | | German            |
|               |                     |                  | | English           |
|               |                     |                  | | Spanish           |
|               |                     |                  | | French            |
|               |                     |                  | | Italian           |
|               |                     |                  | | Japanese          |
|               |                     |                  | | Korean            |
+---------------+---------------------+------------------+---------------------+

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

