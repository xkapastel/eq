Sundial is a protocol for peer-to-peer clocks.

# Protocol
The foundation is gossip with public key encryption: when a node
receives a message, it attempts to decrypt it with every private key
that it knows about. If it cannot decrypt the message, it sends it to
every other node that it knows about.

The content of messages are transactions against a clock. A simple
majority of the compute devoted to the clock determines which
transactions take place. Nodes accrue karma for their contributions to
a clock.

# Bytecode
Sundial bytecode is a [concatenative language](http://tunes.org/~iepos/joy.html).

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
