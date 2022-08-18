//! # Randomizer
//! [![pipeline status](https://gitlab.nebula.technology/libraries/rust/randomizer/badges/main/pipeline.svg)](https://gitlab.nebula.technology/libraries/rust/randomizer/-/commits/main)
//! [![coverage report](https://gitlab.nebula.technology/libraries/rust/randomizer/badges/main/coverage.svg)](https://gitlab.nebula.technology/libraries/rust/randomizer/-/commits/main)
//! [![Latest Release](https://gitlab.nebula.technology/libraries/rust/randomizer/-/badges/release.svg)](https://gitlab.nebula.technology/libraries/rust/randomizer/-/releases)
//! ---
//! Randomizer provides a simple implementation of creating random information, eg random string, and bytes.
//! Thre are many randomization libraries for rust. They are often either complex to use and safe or
//! they are easy to use, but have some significant descrpincies with the code, like using unsafe
//! functions or unwrapping potential Error data. Due to this, i created this library to eliviate
//! those problems.
//!
//! ## Generation.
//! There are 2 (actually 3) types of byte generation.
//! The underneath concept is to generate random bytes for both UTF8 strings and bytes or byte sequences.
//! This generator breaks up the sequence of characters or bytes and from the sequence of
//! combinations it will generate a series of random bytes. if the sequence of bytes are generated
//! from a valid UTF8 string, then each letter can be seperated and used for generating a random
//! sequence of the valid sequences of bytes.
//! This allows for the byte -> UTF8 convertion to be almost infallible.
//!
//! This also means that if the system is supplied with a set of words the generator can randomly
//! put those words together into a sequence.
//!
//! ## Usage
//!
//! **Generate a string**
//! ```rust
//! use randomizer::Randomizer;
//!
//! let string = Randomizer::ALPHANUMERIC(6).string().unwrap();
//! assert_eq!(string.chars().count(), 6);
//! let string = Randomizer::new(6, Some("u")).string().unwrap();
//!
//! assert_eq!(string, "uuuuuu");
//! ```
//!
//! **Generate a byte value**
//! ```rust
//! use randomizer::{Charset, Randomizer};
//!
//! let bytes = Randomizer::new(6, Some(Charset::AnyByte)).bytes().unwrap();
//!
//! assert_eq!(bytes.len(), 6);
//! ```
//!
//! **Generate custom word string**
//! ```rust
//! use randomizer::{Charset, Randomizer};
//!
//! let string = Randomizer::new(6, Some(vec!["foo","bar"])).string().unwrap();
//!
//! assert_eq!(string.chars().count(), 18);
//! ```
//!
//! **Simple String generation with constants**
//! its possible to generate a string using the simplification functions for different types
//! ```rust
//! use randomizer::{Randomizer};
//!
//! let string = Randomizer::ALPHABETICAL(6).string().unwrap();
//!
//! assert_eq!(string.chars().count(), 6);
//! ```
use std::string::FromUtf8Error;
mod constants;
pub mod helpers;

pub use constants::*;
/// Builder for generating random strings or bytes.
///
/// ## Generation.
/// There are 2 (actually 3) types of byte generation.
/// The underneath concept is to generate random bytes for both UTF8 strings and bytes or byte sequences.
/// This generator breaks up the sequence of characters or bytes and from the sequence of
/// combinations it will generate a series of random bytes. if the sequence of bytes are generated
/// from a valid UTF8 string, then each letter can be seperated and used for generating a random
/// sequence of the valid sequences of bytes.
/// This allows for the byte -> UTF8 convertion to be almost infallible.
///
/// This also means that if the system is supplied with a set of words the generator can randomly
/// put those words together into a sequence.
///
///
///
///
/// ```
/// use randomizer::Randomizer;
///
/// let string = Randomizer::ALPHABETICAL(6).string().unwrap();
///
/// assert_eq!(string.chars().count(), 6 );
/// ```
///
#[derive(Debug, Default, Clone)]
pub struct RandomBlock {
    inner: Vec<u8>,
}

/// # Public RandomBlock Implementation
impl RandomBlock {
    /// ## Inner reference
    /// Getting the ['RandomBlock'] inner data as a reference.
    /// ```
    /// use randomizer::RandomBlock;
    ///
    /// let random_block = RandomBlock::default();
    ///
    /// assert_eq!(random_block.inner(), &Vec::new());
    ///
    /// ```
    pub fn inner(&self) -> &Vec<u8> {
        &self.inner
    }

    /// ## Inner mutable reference
    /// Getting the ['RandomBlock'] inner data as a mutable reference.
    /// ```
    /// use randomizer::RandomBlock;
    ///
    /// let mut random_block = RandomBlock::default();
    ///
    /// *random_block.inner_mut() = vec![100u8, 100u8, 100u8];
    ///
    /// assert_eq!(random_block.inner(), &vec![100u8, 100u8, 100u8]);
    ///
    /// ```
    pub fn inner_mut(&mut self) -> &mut Vec<u8> {
        &mut self.inner
    }
}

/// ## Convert Vec<u8> to RandomBlock
/// ```
/// use randomizer::RandomBlock;
///
/// let foo_vec = vec![100u8, 100u8, 100u8];
///
/// let random_block: RandomBlock = foo_vec.clone().into();
///
/// assert_eq!(random_block.inner(), &foo_vec)
/// ```
impl From<Vec<u8>> for RandomBlock {
    fn from(v: Vec<u8>) -> Self {
        RandomBlock { inner: v }
    }
}

/// ## Convert &Vec<u8> to RandomBlock
/// ```
/// use randomizer::RandomBlock;
///
/// let foo_vec = vec![100u8, 100u8, 100u8];
///
/// let random_block: RandomBlock = RandomBlock::from(&foo_vec);
///
/// assert_eq!(random_block.inner(), &foo_vec)
/// ```
impl From<&Vec<u8>> for RandomBlock {
    fn from(v: &Vec<u8>) -> Self {
        RandomBlock { inner: v.clone() }
    }
}

/// ## Convert RandomBlock into String
/// since its not possible to convert RandomBlock into a string by default this will convert the
/// RandomBlock into a ['Result<String, Error>'].
///
/// ```
/// use randomizer::{RandomBlock,Error};
///
/// let foo_vec = vec![100u8, 100u8, 100u8];
///
/// let random_block: RandomBlock = RandomBlock::from(&foo_vec);
/// let string_res: Result<String, Error> = random_block.into();
///
/// assert_eq!(string_res.unwrap(), "ddd")
/// ```
impl From<RandomBlock> for Result<String, Error> {
    fn from(random_block: RandomBlock) -> Result<String, Error> {
        String::from_utf8(random_block.inner).map_err(Error::from)
    }
}

impl From<RandomBlock> for Result<Vec<u8>, Error> {
    fn from(r: RandomBlock) -> Self {
        Ok(r.inner)
    }
}

impl From<RandomBlock> for Vec<u8> {
    fn from(r: RandomBlock) -> Self {
        r.inner
    }
}

pub enum Charset {
    String(String),
    StringSets(Vec<String>),
    Bytes(Vec<u8>),
    ByteSets(Vec<Vec<u8>>),
    AnyByte,
    AnyString,
}

impl Default for Charset {
    fn default() -> Self {
        Charset::AnyByte
    }
}

impl From<String> for Charset {
    fn from(s: String) -> Self {
        Charset::String(s)
    }
}

impl From<Vec<String>> for Charset {
    fn from(v: Vec<String>) -> Self {
        Charset::StringSets(v)
    }
}

impl From<Vec<&str>> for Charset {
    fn from(v: Vec<&str>) -> Self {
        Charset::StringSets(v.into_iter().map(|t| t.to_string()).collect())
    }
}

impl From<&str> for Charset {
    fn from(s: &str) -> Self {
        Charset::String(s.to_string())
    }
}

impl From<&[u8]> for Charset {
    fn from(u: &[u8]) -> Self {
        Charset::Bytes(u.to_vec())
    }
}

#[derive(PartialEq)]
pub enum StrictLimit {
    ByteLimit,
    StringLimit,
    None,
}

pub struct Randomizer {
    length: usize,
    charset: Charset,
    separator: Option<Vec<u8>>,
    strict_limit: StrictLimit,
}

impl Randomizer {
    /// Directly Create a new Randomizer with a seperator.
    ///
    /// ```
    /// use randomizer::Randomizer;
    ///
    /// let string = Randomizer::new(6, Some("u")).string().unwrap();
    ///
    /// assert_eq!(string.chars().count(), 6);
    /// assert_eq!(string, "uuuuuu");
    /// ```
    pub fn new<C: Into<Charset>>(length: usize, charset: Option<C>) -> Randomizer {
        Randomizer {
            length,
            charset: charset.map(|t| t.into()).unwrap_or_default(),
            separator: None,
            strict_limit: StrictLimit::None,
        }
    }

    /// Directly Create a new Randomizer with a seperator.
    ///
    /// ```
    /// use randomizer::Randomizer;
    ///
    /// let string = Randomizer::new_with_separator(6, Some("u"), " ").string().unwrap();
    ///
    /// assert_eq!(string.chars().count(), 11);
    /// assert_eq!(string, "u u u u u u");
    /// ```
    pub fn new_with_separator<C: Into<Charset>, B: Into<Vec<u8>>>(
        length: usize,
        charset: Option<C>,
        sep: B,
    ) -> Randomizer {
        Randomizer {
            length,
            charset: charset.map(|t| t.into()).unwrap_or_default(),
            separator: Some(sep).map(|t| t.into()),
            strict_limit: StrictLimit::None,
        }
    }

    /// Directly Create a random string from Randomizer
    /// This is a simplification of using the RandomBlock to get a string.
    ///
    /// ```
    /// use randomizer::Randomizer;
    ///
    /// let string = Randomizer::ALPHANUMERIC(6).string().unwrap();
    ///
    /// assert_eq!(string.chars().count(), 6);
    ///
    /// let string = Randomizer::new(6, Some("u")).string().unwrap();
    ///
    /// assert_eq!(string.chars().count(), 6);
    /// assert_eq!(string, "uuuuuu");
    /// ```
    pub fn string(&self) -> Result<String, Error> {
        self.rand().and_then(|r| r.into())
    }

    /// Directly Create a random byte vec from the randomizer
    /// This is a simplification of using the RandomBlock to get a byte arrays
    ///
    /// ```
    /// use randomizer::{Charset, Randomizer};
    ///
    /// let bytes = Randomizer::new(6, Some(Charset::AnyByte)).bytes().unwrap();
    ///
    /// assert_eq!(bytes.len(), 6);
    ///
    /// ```
    pub fn bytes(&self) -> Result<Vec<u8>, Error> {
        self.rand().and_then(|r| r.into())
    }

    /// ## Rand - Create RandomBlock
    /// Rand generator creates a RandomBlock that is a convertable struct that allows for the
    /// internal ['Vec<u8>'] into a multiple set of types.
    /// ['Randomizer::rand(&self)'] is commonly used by all simplifications like
    /// ['Randomizer::string(&self)'] and ['Randomizer::bytes(&self)']
    ///
    /// ```
    /// use randomizer::{Charset, Randomizer};
    ///
    /// let bytes = Randomizer::new(6, Some(Charset::AnyByte)).rand().unwrap();
    ///
    /// assert_eq!(bytes.inner().len(), 6);
    /// ```
    pub fn rand(&self) -> Result<RandomBlock, Error> {
        match &self.charset {
            Charset::String(s) => self.rand_string(&s),
            Charset::Bytes(b) => {
                self.rand_byte(Some(b.iter().map(|t| vec![*t]).collect::<Vec<Vec<u8>>>()))
            }
            Charset::AnyByte => self.rand_byte(None),
            Charset::AnyString => self.rand_string(constants::UTF8),
            Charset::ByteSets(v) => self.rand_byte(Some(v.clone())),
            Charset::StringSets(v) => self.rand_string_set(v),
        }
    }

    helpers::local_macros::automated_impl! {
        /// Alphanumeric Randomizer
        (ALPHANUMERIC);
        /// Lowercase Alphanumeric Randomizer
        (ALPHANUMERIC_LOWER);
        /// Uppsercase Alphanumeric Randomizer
        (ALPHANUMERIC_UPPER);
        /// Alphabetical Randomizer
        (ALPHABETICAL);
        /// Lowercase Alphabetical Randomizer
        (ALPHABETICAL_UPPER);
        /// Uppercase Alphabetical Randomizer
        (ALPHABETICAL_LOWER);
        /// Numberical Randomizer
        (NUMERICAL);
        /// Unlimited UTF8
        (UTF8);
    }
}

/// Private implementations
impl Randomizer {
    fn rand_string(&self, charset: &str) -> Result<RandomBlock, Error> {
        let chars: Vec<Vec<u8>> = charset
            .chars()
            .map(|c| c.to_string().as_bytes().to_vec())
            .collect();
        self.rand_byte(Some(chars))
    }

    fn rand_string_set(&self, charset: &Vec<String>) -> Result<RandomBlock, Error> {
        let byte_sets: Vec<Vec<u8>> = charset
            .into_iter()
            .map(|c| c.to_string().as_bytes().to_vec())
            .collect();
        self.rand_byte(Some(byte_sets))
    }

    fn rand_byte(&self, set_opt: Option<Vec<Vec<u8>>>) -> Result<RandomBlock, Error> {
        let mut vec = Vec::new();
        let set = set_opt.unwrap_or_else(|| (0..255u8).into_iter().map(|t| vec![t]).collect());
        let mut idx = 0;
        while idx < self.length {
            if let Some(v) = set.get(fastrand::usize(0..set.len())) {
                vec = vec![vec, v.clone()].concat();
                idx = self.index_add(idx, &vec)?;
                if let Some(sep) = &self.separator {
                    if idx != self.length {
                        vec = vec![vec, sep.clone()].concat();
                        if self.strict_limit != StrictLimit::None {
                            idx = self.index_add(idx, &vec)?;
                        }
                    }
                }
            } else {
                return Err(Error::OverflowUnderflow(
                    "Getting string cause overflow/underflow".to_string(),
                ));
            }
        }

        Ok(vec.into())
    }

    fn index_add(&self, idx: usize, data: &Vec<u8>) -> Result<usize, Error> {
        match self.strict_limit {
            StrictLimit::ByteLimit => Ok(data.len()),
            StrictLimit::StringLimit => String::from_utf8(data.clone())
                .map_err(Error::from)
                .map(|t| t.chars().count()),
            StrictLimit::None => Ok(idx + 1),
        }
    }
}

#[derive(Debug)]
pub enum Error {
    OverflowUnderflow(String),
    ByteConversionFailedForUTF8(FromUtf8Error),
    CharsetMismatch(String),
}

impl From<FromUtf8Error> for Error {
    fn from(e: FromUtf8Error) -> Self {
        Error::ByteConversionFailedForUTF8(e)
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    pub fn test_ascii_generation() {
        let string = Randomizer::ALPHABETICAL(6).string().unwrap();
        println!("{}", string);
        assert_eq!(string.chars().count(), 6);
    }

    #[test]
    pub fn test_unicode_generation() {
        let charset = "ó❤⚙";
        let string = Randomizer::new(6, Some(charset)).string().unwrap();
        println!("{}", string);
        assert_eq!(string.chars().count(), 6);
    }

    #[test]
    pub fn test_unicode_generation_single() {
        let charset = "⚙";
        println!("{:?}", charset.as_bytes());
        let charset = "1⚙2";
        println!("{:?}", charset.as_bytes());
        let string = Randomizer::new(6, Some(charset)).string().unwrap();
        println!("{}", string);
        assert_eq!(string.chars().count(), 6);
    }

    #[test]
    pub fn test_unicode_generation_large() {
        let charset = "ó❤⚙";
        let string = Randomizer::new(12, Some(charset)).string().unwrap();
        println!("'{}'", string);
        assert_eq!(string.chars().count(), 12);
    }

    #[test]
    pub fn test_new_with_separator() {
        let string = Randomizer::new_with_separator(6, Some("u"), " ")
            .string()
            .unwrap();
        println!("{:?}", string);
        assert_eq!(string.chars().count(), 11);
        assert_eq!(string, "u u u u u u");
    }

    #[test]
    pub fn test_rand() {
        let bytes = Randomizer::new(6, Some(Charset::AnyByte)).rand().unwrap();

        println!("{:?}", bytes);

        assert_eq!(bytes.inner().len(), 6);
    }

    #[test]
    pub fn test_randomblock_to_result() {
        let foo_vec = vec![100u8, 100u8, 100u8];

        let random_block: RandomBlock = RandomBlock::from(&foo_vec);
        let string_res: Result<String, Error> = random_block.into();

        assert_eq!(string_res.unwrap(), "ddd")
    }
}
