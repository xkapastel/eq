Sundial is a protocol for peer-to-peer clocks.

# Protocol
The foundation is gossip with public key encryption: when a node
receives a message, it attempts to decrypt it with every private key
that it knows about. If it cannot decrypt the message, it sends it to
every other node that it knows about.

A clock is a collection of computers that agree on a sequence of
events.

The content of messages are transactions against a clock. A simple
majority of the compute devoted to the clock determines which
transactions take place. Nodes accrue karma for their contributions to
a clock.

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
