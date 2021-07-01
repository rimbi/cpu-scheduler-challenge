1. What is the runtime complexity of your solution?

O(n2*log(n))

2. Rewrite the `execution_order` function signature to be Rust idiomatic.

```rust
fn execution_order(tasks: &[Task]) -> Result<Vec<u64>, ()>;
```

3. Discuss any potential improvements you might make but don't have time to do.
- There might be a way to do the sorting once