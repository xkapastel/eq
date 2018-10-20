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
    [A] a = A
    [A] b = [[A]]
[A] [B] c = [A B]
    [A] d = [A] [A]
    [A] e =
[A] [B] f = [B] [A]
```
