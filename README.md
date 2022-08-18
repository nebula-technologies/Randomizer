# Randomizer
---
Randomizer provides a simple implementation of creating random information, eg random string, and bytes.
Thre are many randomization libraries for rust. They are often either complex to use and safe or
they are easy to use, but have some significant descrpincies with the code, like using unsafe
functions or unwrapping potential Error data. Due to this, i created this library to eliviate
those problems.

## Generation.
There are 2 (actually 3) types of byte generation.
The underneath concept is to generate random bytes for both UTF8 strings and bytes or byte sequences.
This generator breaks up the sequence of characters or bytes and from the sequence of
combinations it will generate a series of random bytes. if the sequence of bytes are generated
from a valid UTF8 string, then each letter can be seperated and used for generating a random
sequence of the valid sequences of bytes.
This allows for the byte -> UTF8 convertion to be almost infallible.

This also means that if the system is supplied with a set of words the generator can randomly
put those words together into a sequence.

## Usage

**Generate a string**

```rust
use randomizer::Randomizer;

let string = Randomizer::ALPHANUMERIC(6).string().unwrap();
assert_eq!(string.chars().count(), 6);
let string = Randomizer::new(6, Some("u")).string().unwrap();

assert_eq!(string, "uuuuuu");
```

**Generate a byte value**

```rust
use randomizer::{Charset, Randomizer};

let bytes = Randomizer::new(6, Some(Charset::AnyByte)).bytes().unwrap();

assert_eq!(bytes.len(), 6);
```

**Generate custom word string**

```rust
use randomizer::{Charset, Randomizer};

let string = Randomizer::new(6, Some(vec!["foo","bar"])).string().unwrap();

assert_eq!(string.chars().count(), 18);
```

**Simple String generation with constants**
its possible to generate a string using the simplification functions for different types

```rust
use randomizer::{Randomizer};

let string = Randomizer::ALPHABETICAL(6).string().unwrap();

assert_eq!(string.chars().count(), 6);
```
