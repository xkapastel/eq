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
[A] [B] g = [A] [B] g
    [A] h = [A] h
```

```
Γ                 :- [A]
[A]                = [B]
---------------------------------------    Eval
Γ                 :- [B]

Γ                 :- [[F] [G] g]
Γ                 :- Π(i;N).[Ai] F
---------------------------------------    App
Γ                 :- Π(i;N).[Ai] G

Γ                 :- [Π(i;N).[xi] F h]
Γ, Π(i;N).[xi] F  :- Π(i;N).[xi] G
---------------------------------------    Abs
Γ                 :- [[F] [G] g]

Γ                 :- [Π(i;N).[xi] F h]
Γ, Π(i;N).[xi] F  :- [Π(i;N).[xi] G h]
---------------------------------------    Prop
Γ                 :- [[[F] [G] g] h]

---------------------------------------    Loop
Γ                 :- [[[A] h] h]
```
