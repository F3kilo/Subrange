# Subranges

Interval struct and collection for free intervals.

## Example

```rust
use subranges::FreeIntervals;

fn main() {
    // new collection with free interval
    let free = (0..100).into();
    let mut collection = FreeIntervals::new(free);

    // fill subrange of free interval
    let first = collection.take_exact(32).unwrap();
    println!("first: {first}"); // output: first: [0, 32)

    // fill other subrange of free interval with alignment
    let aligned = collection.take_exact_aligned(32, 10).unwrap();
    println!("aligned: {aligned}"); // output: aligned: [40, 72)

    // no free subrange with `length == 40`.
    let none = collection.take_exact(40);
    assert!(none.is_none());

    // free `first` intervals with `length == 32`,
    // it connects with padding with `length == 8` from aligned interval.
    collection.insert(first);

    // now we have subrange with `length == 40`.
    let some = collection.take_exact(40).unwrap();
    println!("some: {some}"); // output: some: [0, 40)
}
```