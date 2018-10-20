Sundial is a distributed virtual machine.

# Bytecode
Sundial bytecode is a [concatenative language](http://tunes.org/~iepos/joy.html).

``` 
    [A] a = A
    [A] b = [[A]]
[A] [B] c = [A B]
    [A] d = [A] [A]
    [A] e =
[A] [B] f = [B] [A]
```
