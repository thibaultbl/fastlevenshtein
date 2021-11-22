.. contents ::

Introduction
------------
Rust implementation of levensthein distance (https://en.wikipedia.org/wiki/Levenshtein_distance).

Installation
------------

::

   pip install fastlevensthein


Usage
------------

::

   from fastlevensthein import levensthein
   levensthein("string1", "string2")
   >>> 1

   from fastlevensthein import levensthein_list
   levensthein_list(["string1", "string2"], "string3")
   >>> [1, 1]


License
-------

fastlevenshtein is free software; you can redistribute it and/or modify it
under the terms of the GNU General Public License as published by the Free
Software Foundation; either version 2 of the License, or (at your option)
any later version.

See the file COPYING for the full text of GNU General Public License version 2.
