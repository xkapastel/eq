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
    [A] g = [A] g
    [A] h = [A] h
[A] [B] i = [A] [B] i
```

```
Γ                 :- [A]
[A]                = [B]
---------------------------------------    reduce
Γ                 :- [B]

---------------------------------------    top
Γ                 :- [[A] g]

Γ                 :- [[F] [G] i]
Γ                 :- Π(i;N).[Ai] F
---------------------------------------    apply
Γ                 :- Π(i;N).[Ai] G

Γ                 :- [Π(i;N).[xi] F h]
Γ, Π(i;N).[xi] F  :- Π(i;N).[xi] G
---------------------------------------    abstract
Γ                 :- [[F] [G] i]

Γ                 :- [Π(i;N).[xi] F h]
Γ, Π(i;N).[yi] F  :- [Π(i;N).[yi] G h]
---------------------------------------    well-formed
Γ                 :- [[[F] [G] i] h]
```
