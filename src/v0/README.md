# v0

This is an interpreter for

```
        [A] %app = A
        [A] %box = [[A]]
    [A] [B] %cat = [A B]
        [A] %cpy = [A] [A]
        [A] %drp =
    [A] [B] %swp = [B] [A]
        [A] %fix = [[A] %fix A]
        [A] %run = { A }
{ E [F] %jmp K } = [{ E }] [{ K }] F
```

based on linked lists.
