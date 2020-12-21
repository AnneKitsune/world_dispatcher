# World Dispatcher
The system part of a full ECS (Entity-Component-System).
It also contains a `World` structure, which holds the game data used by systems.
The `Dispatcher` is used to execute systems in parallel and in an optimised
order.

# Usage
Add the following to you Cargo.toml file:
```
world_dispatcher = "0.99.0"
```

Use it like so:
```rust
use world_dispatcher::*;
fn main() {
    #[derive(Default)]
    pub struct A;

    let mut world = World::default();

    let sys = (|_comps: &A| Ok(())).system();

    let mut dispatch = DispatcherBuilder::new().add_system(sys).build(&mut world);
    dispatch.run_seq(&world).unwrap();
    dispatch.run_seq(&world).unwrap();
    dispatch.run_seq(&world).unwrap();

    assert!(world.get::<A>().is_ok());
}
```

It is also possible to convert most functions into systems.
There are four requirements for this:
- Take only & and &mut references as arguments
- Return a SystemResult
- Use all & references before all &mut references in the arguments.
- Do not use the same type twice in the arguments.
- All types in the arguments must implement `Default`. If they don't, use
`&/&mut Option<YourType>` instead.
```rust
use world_dispatcher::*;

#[derive(Default)]
pub struct A;
#[derive(Default)]
pub struct B;
#[derive(Default)]
pub struct C;
pub struct D;

fn system_function(_a: &A, _b: &B, _c: &mut C, d: &mut Option<D>) -> SystemResult {
    assert!(d.is_some());
    Ok(())
}

fn main() {
    let mut world = World::default();
    // Will automatically create A, B, C, Option<D>::None inside of world.
    let mut dispatch = DispatcherBuilder::new().add(system_function).build(&mut world);
    // Let's assign a value to D.
    *world.get_mut::<Option<D>>().unwrap() = Some(D);

    dispatch.run_seq(&world).unwrap();
    dispatch.run_seq(&world).unwrap();
    dispatch.run_seq(&world).unwrap();

    assert!(world.get::<Option<D>>().unwrap().is_some());
}
```

### Focks Team Information
Maintainer: Jojolepro

Contact: jojolepro [at] jojolepro [dot] com

Commercial license available: yes

[Focks Team Website](https://jojolepro.com/focks)

### Licence
AGPL-3.0. You can buy commercial licenses [here](https://jojolepro.com/focks/).

See full license in LICENSE file.

Copyright (C) 2020 Jojolepro.
