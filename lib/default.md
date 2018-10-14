Common names for the primitive functions.

```eq
:app %app
:box %box
:cat %cat
:copy %cpy
:drop %drp
:swap %swp
:fix %fix
:shift %jmp
:forall %all
:set %set
:num %num
:+ %add
:* %mul
:- %neg
:/ %inv
:max %max
:min %min
:exp %exp
:log %log
:cos %cos
:sin %sin
:abs %abs
:ceil %cel
:floor %flr
```

Products.

```eq
:pair box swap box swap cat

:app1 box cat app
:app2 pair app1 app
:app3 pair app2 app

:box1 swap box swap
:box2 pair box1 app
:box3 pair box2 app

:drop1 swap drop
:drop2 pair drop1 app
:drop3 pair drop2 app

:swap1 swap
:swap2 pair swap app1
:swap3 pair swap2 app1

:copy1 swap copy swap2
:copy2 pair copy1 app
:copy3 pair copy2 app

:fst app drop
:snd app drop1

:case swap2 app
:bind box1 cat
:call1 swap app
:inl [drop1 call1] bind
:inr [drop2 call1] bind

:bac swap2
:cbad swap3
:badc pair swap2 swap2 app2
```
