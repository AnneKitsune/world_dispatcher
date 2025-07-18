Support an Open Source Developer! :hearts:  

[![Become a patron](https://c5.patreon.com/external/logo/become_a_patron_button.png)](https://www.patreon.com/annekitsune)

# World Dispatcher
The system part of a full ECS (Entity-Component-System).

It also contains a `World` structure, which holds the game data used by systems,
as well as the `Dispatcher` that is used to execute systems in parallel and in 
an optimised order.

# Why would you use this ECS library?

* Compatible with all platforms, including WASM!
* Fast enough on *every* operation, not just iteration.
* Minimal amount of dependencies.
* Small code size.
* Stable, tested, benchmarked, 100% completed.

# Usage
Add the following to you Cargo.toml file:
```
world_dispatcher = "*"
```

Use it like so:
```rust
use world_dispatcher::*;
fn main() {
    #[derive(Default)]
    pub struct A;

    let mut world = World::default();

    let sys = (|_comps: &A| {}).system();

    let mut dispatch = DispatcherBuilder::new().add_system(sys).build(&mut world);
    dispatch.run_seq(&world);
    dispatch.run_seq(&world);
    dispatch.run_seq(&world);
}
```

It is also possible to convert most functions into systems.

There are four requirements for this:
- Take only & and &mut references as arguments
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

fn system_function(_a: &A, _b: &B, _c: &mut C, d: &mut Option<D>) {
    assert!(d.is_some());
}

fn main() {
    let mut world = World::default();
    // Will automatically create A, B, C, Option<D>::None inside of world.
    let mut dispatch = DispatcherBuilder::new().add(system_function).build(&mut world);
    // Let's assign a value to D.
    *world.get_mut::<Option<D>>() = Some(D);

    dispatch.run_seq(&world);
    dispatch.run_seq(&world);
    dispatch.run_seq(&world);

    assert!(world.get::<Option<D>>().is_some());
}
```

If you need more than 12 system parameters, there is a feature called `big_systems`
which will bump that limit to 22. **First** compilation time will be around 10
seconds if using it. Following compilations will be instant.

### Maintainer Information

* Maintainer: Anne Kitsune
* Contact: annekitsunefox [at] gmail [dot] com
* Website: [annekitsune.com](https://annekitsune.com)
* Patreon: [patreon](https://patreon.com/annekitsune)

