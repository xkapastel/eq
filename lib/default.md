Utilities.

```eq
:miss swap box cat app
:bind swap box swap cat
:bca box swap box swap cat miss
```

Products & coproducts.

```eq
:unit []
:pair box swap box swap cat
:fst app drop
:snd app swap drop
:case bca app
:inl box [swap drop swap app] cat
:inr box [bca drop swap app] cat
```

Monoids for addition and multiplication. There's an article on
[interface-passing
style](https://common-lisp.net/~frideau/lil-ilc2012/lil-ilc2012.html)
that explains the idea.

```eq
:m+ [app +] [drop 0] case
:m* [app *] [drop 1] case
```
