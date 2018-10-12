Sundial is a peer-to-peer virtual machine.

# Actors
The foundation is gossip with public key encryption: when an actor
receives a message, it attempts to decrypt it. If it can't, it passes
it on to other actors.

# Bytecode
Sundial bytecode is a [concatenative language](http://tunes.org/~iepos/joy.html).

```
         [A] %app = A
         [A] %box = [[A]]
     [B] [A] %cat = [B A]
         [A] %cpy = [A] [A]
         [A] %drp =
     [B] [A] %swp = [A] [B]
         [A] %fix = [[A] %fix A]
         [A] %run = { A }
 { E [F] %jmp K } = { [E] [K] F }
```
