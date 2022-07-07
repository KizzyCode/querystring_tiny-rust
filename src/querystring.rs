//! Implements a trivial query string decoder

use crate::error::Error;
use std::{collections::BTreeMap, iter::FromIterator, ops::Deref};

/// A query string
///
/// ## Warning
/// The query parser is pretty simple and basically parses any `key` or `key=` or `key=value` component without further
/// validation.
///
/// The following rules apply:
///  - the query string _MUST NOT_ begin with a `?` – it's not a bug, it's a feature: this allows the parser to parse raw
///    query strings in the body (e.g. from HTML forms)
///  - keys don't need a value (i.e. `key0&key1` is valid)
///  - keys can have an empty value (i.e. `key0=&key1=` is valid)
///  - keys can have a non-empty value (i.e. `key0=value0&key1=value1` is valid)
///  - empty keys/key-value pairs are ignored (i.e. `&` evaluates to `[]`, `key0&&key1` evaluates to
///    `["key0": "", "key1": ""]` and `=value0&key1=value1&` evaluates to `["key1": "value1"]`)
#[derive(Debug, Clone, Default)]
pub struct QueryString {
    /// The key-value fields of the query string
    fields: BTreeMap<Vec<u8>, Vec<u8>>,
}
impl QueryString {
    /// Creates a new header field map
    pub fn new() -> Self {
        Self { fields: BTreeMap::new() }
    }
    /// Decodes a query string
    pub fn decode(source: &[u8]) -> Result<Self, Error> {
        // Parse the query components
        let (mut source, mut fields) = (source.iter().copied(), BTreeMap::new());
        while source.len() > 0 {
            // Read the next pair
            let mut pair = (&mut source).take_while(|&b| b != b'&');
            let key: Vec<_> = (&mut pair).take_while(|&b| b != b'=').collect();
            let value: Vec<_> = (&mut pair).collect();

            // Insert the key if it is not empty
            if !key.is_empty() {
                fields.insert(key, value);
            }
        }
        Ok(Self { fields })
    }

    /// Gets the value for the field with the given name
    pub fn get<T>(&self, name: T) -> Option<&[u8]>
    where
        T: AsRef<[u8]>,
    {
        self.fields.get(name.as_ref()).map(|s| s.as_ref())
    }
    /// Sets the value for a fiels with the given name
    pub fn set<A, B>(&mut self, name: A, value: B)
    where
        A: Into<Vec<u8>>,
        B: Into<Vec<u8>>,
    {
        self.fields.insert(name.into(), value.into());
    }

    /// Encodes the query string
    pub fn encode(&self) -> Vec<u8> {
        // Serialize all elements
        let mut serialized = Vec::new();
        for (key, value) in self.fields.iter() {
            // Write delimiter
            if !serialized.is_empty() {
                serialized.extend(b"&");
            }

            // Write key and value
            serialized.extend(key);
            if !value.is_empty() {
                serialized.extend(b"=");
                serialized.extend(value);
            }
        }
        serialized
    }
}
impl Deref for QueryString {
    type Target = BTreeMap<Vec<u8>, Vec<u8>>;

    fn deref(&self) -> &Self::Target {
        &self.fields
    }
}
impl<K, V> FromIterator<(K, V)> for QueryString
where
    K: Into<Vec<u8>>,
    V: Into<Vec<u8>>,
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(pairs: T) -> Self {
        let fields = pairs.into_iter().map(|(k, v)| (k.into(), v.into())).collect();
        Self { fields }
    }
}
impl IntoIterator for QueryString {
    type Item = <BTreeMap<Vec<u8>, Vec<u8>> as IntoIterator>::Item;
    type IntoIter = <BTreeMap<Vec<u8>, Vec<u8>> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.fields.into_iter()
    }
}
