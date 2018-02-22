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
| Japan    | ja         |
+----------+------------+

Supported builtin entities
--------------------------

+---------------+---------------------+---------------------+
| Entity        | Identifier          | Supported languages |
+===============+=====================+=====================+
| AmountOfMoney | snips/amountOfMoney | | English           |
|               |                     | | French            |
|               |                     | | German            |
|               |                     | | Spanish           |
|               |                     | | Korean            |
+---------------+---------------------+---------------------+
| Time          | snips/datetime      | | English           |
|               |                     | | Spanish           |
|               |                     | | French            |
|               |                     | | Korean            |
|               |                     | | German            |
+---------------+---------------------+---------------------+
| Duration      | snips/duration      | | English           |
|               |                     | | Spanish           |
|               |                     | | French            |
|               |                     | | Korean            |
|               |                     | | German            |
+---------------+---------------------+---------------------+
| Number        | snips/number        | | English           |
|               |                     | | Spanish           |
|               |                     | | French            |
|               |                     | | Korean            |
|               |                     | | German            |
+---------------+---------------------+---------------------+
| Ordinal       | snips/ordinal       | | English           |
|               |                     | | Spanish           |
|               |                     | | French            |
|               |                     | | Korean            |
|               |                     | | German            |
+---------------+---------------------+---------------------+
| Percentage    | snips/percentage    | | English           |
|               |                     | | Spanish           |
|               |                     | | French            |
|               |                     | | German            |
+---------------+---------------------+---------------------+
| Temperature   | snips/temperature   | | English           |
|               |                     | | Spanish           |
|               |                     | | French            |
|               |                     | | Korean            |
|               |                     | | German            |
+---------------+---------------------+---------------------+

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

