#![allow(dead_code)]

use std::fmt::Display;

#[derive(Debug)]
struct HashMap<K, V> {
    pub buckets: Vec<Option<Bucket<K, V>>>,
}

#[derive(Debug)]
struct Bucket<K, V> {
    key: K,
    val: V,
    next: Option<Box<Bucket<K, V>>>,
}

impl<K, V> HashMap<K, V>
where
    V: Clone,
    K: ToString + Display,
{
    fn new() -> Self {
        Self { buckets: vec![] }
    }

    fn insert(&mut self, k: K, v: V) -> Result<(), HashMapErr> {
        let index = self.hash(&k)?;

        // Extend storage if we reach index limit
        if self.buckets.len() < index {
            while self.buckets.is_empty() || self.buckets.len() - 1 < index {
                self.buckets.push(None);
            }
        }

        if self.buckets[index].is_some() {
            // TODO: append linked list at location
            return Err(HashMapErr::Collision);
        }

        // TODO: start a link list if item already exist at this position
        self.buckets[index] = Some(Bucket {
            key: k,
            val: v,
            next: None,
        });

        Ok(())
    }

    fn get(&self, key: &K) -> Result<Option<V>, HashMapErr> {
        let i = self.hash(key)?;
        let b = self.buckets[i].as_ref();

        if let Some(b) = b {
            Ok(Some(b.val.clone()))
        } else {
            Ok(None)
        }
    }

    // TODO: replace with something more robust
    fn hash(&self, key: &K) -> Result<usize, HashMapErr> {
        Ok(key.to_string().len() - 1)
    }
}

#[derive(Debug)]
enum HashMapErr {
    Collision,
}

#[test]
fn hash_map_test() {
    let mut d: HashMap<String, usize> = HashMap::new();

    d.insert("hello".to_string(), 5).unwrap();
    d.insert("ggwp".to_string(), 4).unwrap();

    println!("{:?}", d);
    println!("{:?}", d.get(&"hello".to_string()));
}
