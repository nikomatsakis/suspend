Two basic operations:

-`layer(closure: for<'a> fn(&'a Basis) -> Derived<'a>) -> Suspended<D>`
- with

```
GlobalCtxt<'global_interners, 'gcx, 'gcx>
DroplessArena
CtxtInterners<'arena>
TyCtxt<'interners, 'gcx, 'arena>
InferCtxt<'interners, 'gcx, 'arena>
```

Layer: L
- has some stored data of type T at fixed address `Ptr<T>`
- layer: `(&'a T) -> U<'a>`
