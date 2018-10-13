Sundial is a peer-to-peer virtual machine.

# Processes
The foundation is process calculus, networked with gossip and public
key encryption.

A *read port* is a public/private keypair used for encryption and
decryption. A *write port* is a public/private keypair used for
signing and verification.

Suppose machine A is running a process Pa that writes to port W, and
machine B is running a process Pb that reads from port R. Then A
encrypts Pa's value with R's public key, signs it with W's private
key, and sends it to B. B verifies it with W's public key, decrypts it
with R's private key and gives it to Pb.

A and B are connected by an Ethernet-like network where nodes attempt
to decrypt messages in the above manner, and pass along messages that
they cannot. When a node connects to the swarm, for each read port it
wishes to read from, it broadcasts a *beep*: a triple of the hash of
the read port's public key, its own IP address, and a randomly
generated *position* of the same bitlength as the hash.

The routing algorithm is meant to be similar to
[Kademlia](https://en.wikipedia.org/wiki/Kademlia), substituting the
hash of a public key for the key in a table.

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
