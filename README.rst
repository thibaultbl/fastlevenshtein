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

First param is either a list or a str.

It will be compared to the second param that is a str.

The third param is the maximum value of levensthein. 
For example, if the third param is 3, if the levensthein value reach at least 3, the function with exit and return 3 as value.
This third param is used to increase computation time when you are only interested in string that are close to each other.
If you do not want to use early exit, you can just use a really high value for this param

::

   from fastlevensthein import levensthein
   levensthein("string1", "string2", 999) # 999 so no early exit
   >>> 1

   from fastlevensthein import levensthein_list
   levensthein_list(["string1", "string2"], "string3", 999)
   >>> [1, 1]

   from fastlevensthein import levensthein_list
   levensthein_list(["aaaaaaaa", "bbbbbbbb"], "ccccccc", 3) # early exit at 3 maximum value for levensthein distance
   >>> [3, 3]


License
-------

fastlevenshtein is free software; you can redistribute it and/or modify it
under the terms of the GNU General Public License as published by the Free
Software Foundation; either version 2 of the License, or (at your option)
any later version.

See the file COPYING for the full text of GNU General Public License version 2.
