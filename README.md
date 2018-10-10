Eq is a blockchain database.

# Bytecode
Eq bytecode is a [concatenative language](http://tunes.org/~iepos/joy.html).

```
         [A] app  = A
         [A] box  = [[A]]
     [A] [B] cat  = [A B]
         [A] copy = [A] [A]
         [A] drop =
     [A] [B] swap = [B] [A]
         [A] fix  = [[A] fix A]
         [A] run  = { A }
{ E [F] shift K } = { [E] [K] F }
```

# Notebooks
Notebooks are Markdown files containing Eq code blocks.
