Common names for the primitive functions.

```sundial
:app a
:box b
:cat c
:copy d
:drop e
:swap f
```

Products and coproducts.

```sundial
:pair box swap box swap cat
:fst app drop
:snd app swap drop

:app1 box cat app
:app2 pair app1 app
:app3 pair app2 app

:box1 swap box swap
:box2 pair box1 app
:box3 pair box2 app

:swap1 swap
:swap2 pair swap app1
:swap3 pair swap2 app1

:drop1 swap drop
:drop2 pair drop1 app
:drop3 pair drop2 app

:copy1 swap copy swap2
:copy2 pair copy1 app
:copy3 pair copy2 app

:case swap2 app
:bind box1 cat
:call swap app
:inl [drop1 call] bind
:inr [drop2 call] bind
```

Useful shuffling.

```sundial
:bac swap2
:cbad swap3
:badc pair swap2 swap2 app2
```
