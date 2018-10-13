Sundial is a peer-to-peer virtual machine.

# Processes
The foundation is process calculus, networked with gossip and
encryption.

A *channel* is a symmetric key used to encrypt blocks. A process may
read or write from a channel: a read will wait for a packet on the
network that can be decrypted with that key, and a write will encrypt
a packet with the key and send it over the network.

The routing algorithm is meant to be similar to
[Kademlia](https://en.wikipedia.org/wiki/Kademlia).

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
