# Names

- See [Cells](cells.md) for more information on Cells.
- See [Morphs](morphs.md) for more information on Morphs.

## What's in a Name?

A Name is a special kind of Cell.  A Name contains the location of some
other Cell, which is typically a memory address or file handle, but
might also be a Linux pipe, or a TCP socket <???> it might even simply
contain the other Cell, if that Cell is a simple value. </???>

<???>
Referring to a Name applies an appropriate Morph to it, if there is a
Morph which results in the desired type of Cell.  If more than one Morph
might give a useful result.  For instance, (+ 2 2) evaluates "+" to the
Morph which + refers to.
</???>

Names have a predefined set of possible Morphs; of course, 
