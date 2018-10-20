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
Γ                 :- pi(i;N).[Ai] F
---------------------------------------    apply
Γ                 :- pi(i;N).[Ai] G

Γ                 :- [pi(i;N).[xi] F h]
Γ, pi(i;N).[yi] F :- pi(i;N).[yi] G
---------------------------------------    abstract
Γ                 :- [[F] [G] i]

Γ                 :- [pi(i;N).[xi] F h]
Γ, pi(i;N).[yi] F :- [pi(i;N).[yi] G h]
---------------------------------------    well-formed
Γ                 :- [[[F] [G] i] h]
```
