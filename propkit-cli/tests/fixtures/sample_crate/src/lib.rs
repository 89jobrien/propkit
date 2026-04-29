use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UserId(pub u64);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Priority(pub u8);

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub name: String,
    pub value: i32,
}

#[derive(Debug, Default)]
pub struct Empty;

impl fmt::Display for UserId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "user-{}", self.0)
    }
}

impl FromStr for UserId {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.strip_prefix("user-")
            .and_then(|n| n.parse().ok())
            .map(UserId)
            .ok_or_else(|| format!("invalid user id: {s}"))
    }
}

pub fn normalize(x: f64) -> f64 {
    x
}

pub fn encode(data: &[u8]) -> Vec<u8> {
    data.to_vec()
}

pub fn decode(data: &[u8]) -> Vec<u8> {
    data.to_vec()
}

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn sort_items(items: &mut Vec<i32>) {
    items.sort();
}
