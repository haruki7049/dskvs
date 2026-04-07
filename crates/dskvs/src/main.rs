use std::str::FromStr;

use clap::Parser;
use thiserror::Error;

fn main() -> anyhow::Result<()> {
    _ = CLIArgs::parse();

    let record = Record::new(Key::from_str("key")?, Value::string("value".to_string()));
    let record_record = Record::new(Key::from_str("key")?, Value::record(record.clone()));
    let mut store = KVS::default();
    store.push(record.clone());
    store.push(record.clone());
    store.push(record.clone());
    store.push(record);
    store.push(record_record);

    mainloop(store)?;

    Ok(())
}

#[derive(Debug, Parser)]
#[command(about, author, version)]
struct CLIArgs;

fn mainloop(store: KVS) -> anyhow::Result<()> {
    store.into_iter().for_each(|v| println!("{:?}", v));

    Ok(())
}

/// A Key-Value Store.
#[derive(Debug, Default)]
struct KVS {
    inner: Vec<Record>,
}

trait Store {
    fn push(&mut self, record: Record);
}

impl Store for KVS {
    fn push(&mut self, record: Record) {
        self.inner.push(record);
    }
}

impl IntoIterator for KVS {
    type Item = Record;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

#[derive(Debug, Error)]
enum StoreError {}

/// Record which has a key and a value.
#[derive(Clone, Debug)]
struct Record {
    pub key: Key,
    pub value: Value,
}

impl Record {
    fn new(key: Key, value: Value) -> Self {
        Self { key, value }
    }
}

/// Key type which has a String to pull a value from a record
#[derive(Clone, Debug)]
struct Key {
    inner: String,
}

impl std::str::FromStr for Key {
    type Err = KeyFromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let inner = s.to_string();
        Ok(Self { inner })
    }
}

#[derive(Debug, Error)]
enum KeyFromStrError {}

/// Value type which is contained to a record
#[derive(Clone, Debug)]
enum Value {
    String(String),
    Integer(isize),
    Float(f64),
    Record(Box<Record>),
}

impl Value {
    fn string(inner: String) -> Self {
        Self::String(inner)
    }

    fn integer(inner: isize) -> Self {
        Self::Integer(inner)
    }

    fn float(inner: f64) -> Self {
        Self::Float(inner)
    }

    fn record(inner: Record) -> Self {
        Self::Record(Box::new(inner))
    }
}
