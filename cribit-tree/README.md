# Cribit tree

Inspired by [tzaeschke](https://github.com/tzaeschke) in PH-trees.

CritBit is a multi-dimensional OR arbitrary length crit-bit tree.

Cribit trees are very space efficient due to prefix-sharing and suitable for
multi-dimensional data with low dimensionality (e.g. less than 10 dimensions or so).
They are also stable, that means unlike kD-trees or quadtrees they do not require
rebalancing, this makes update performance much more predictable.

There is 1 1D-version and a kD-version (kD: k-dimensional).
The 1D version supports keys with arbitrary length (e.g. 256bit), the kD-version
supports k-dimensional keys with a maximum length of 64 bit per dimension. 

Both tree versions use internally the same methods, except for the range queries.
For range queries, the 1D version interprets the parameters as one minimum and one
maximum value. For kD queries, the parameters are interpreted as arrays of
minimum and maximum values (i.e. the low left and upper right 
corner of a query (hyper-)rectangle). 

All method ending with 'KD' are for k-dimensional use of the tree, all other methods are for
1-dimensional use. Exceptions are the len(), print_tree() and similar methods, which work  for
all dimensions. 
<!-- 
In order to store floating point values, please convert them to 'long' with
BitTools.toSortableLong(...), also when supplying query parameters.
Extracted values can be converted back with BitTools.toDouble() or toFloat(). -->