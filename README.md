# DPS
DPS provides a collection of algorithms useful in 
digital signal processing (mainly audio),
as well as a cool and shiny way of composing them.

## Basic usage

### Using a single processor
```rust
  let mut buffer = [1.0; 512];

  dps::gain(0.5.into()).process(&mut [&mut buffer]);
```

### Chaining processors
```rust
  let mut buffer = [1.0; 512];

  dps::hard_clip(0.8.into()) // apply hard clip
      .then(dps::gain(0.5.into())) // then apply gain
      .process(&mut [&mut buffer]); // finally let's apply the processing chain to the output buffer
```

### Separate inputs and outputs
```rust
  let input = [1.0; 512];
  let mut output = [0.0; 512];

  dps::copy([&input])
      .then(dps::gain(0.5.into()))
      .process(&mut [&mut output]);
```
