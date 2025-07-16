use std::net::IpAddr;
use std::sync::Arc;
use parking_lot::RwLock;
use ip_network::IpNetwork;
use ip_network_table::IpNetworkTable;
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use serde::ser::SerializeStruct;
use std::fmt;
use crate::ip_lookup::types::{IpCategory, IpRange, Result, IpRangeError};
use std::collections::HashMap;
use std::path::{Path};
use std::fs;

/// A radix tree for efficient IP address lookups.
/// 
/// This structure uses separate trees for IPv4 and IPv6 addresses to optimize
/// memory usage and lookup performance.
pub struct RadixTree {
    v4_table: IpNetworkTable<IpCategory>,
    v6_table: IpNetworkTable<IpCategory>,
    metadata: HashMap<String, String>,
    stats: LookupStats,
}

// Implement Debug manually for RadixTree
impl fmt::Debug for RadixTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RadixTree")
            .field("v4_entries", &self.v4_table.iter().count())
            .field("v6_entries", &self.v6_table.iter().count())
            .field("metadata", &self.metadata)
            .field("stats", &self.stats)
            .finish()
    }
}

// Implement Default manually for RadixTree
impl Default for RadixTree {
    fn default() -> Self {
        Self {
            v4_table: IpNetworkTable::new(),
            v6_table: IpNetworkTable::new(),
            metadata: HashMap::new(),
            stats: LookupStats::default(),
        }
    }
}

// Manual implementation of Serialize for RadixTree
impl Serialize for RadixTree {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Convert IpNetworkTable to a serializable format (Vec of (network, category))
        let v4_entries: Vec<(String, IpCategory)> = self.v4_table
            .iter()
            .map(|(net, &cat)| (net.to_string(), cat))
            .collect();
            
        let v6_entries: Vec<(String, IpCategory)> = self.v6_table
            .iter()
            .map(|(net, &cat)| (net.to_string(), cat))
            .collect();

        let mut state = serializer.serialize_struct("RadixTree", 4)?;
        state.serialize_field("v4_entries", &v4_entries)?;
        state.serialize_field("v6_entries", &v6_entries)?;
        state.serialize_field("metadata", &self.metadata)?;
        state.serialize_field("stats", &self.stats)?;
        state.end()
    }
}

// Manual implementation of Deserialize for RadixTree
impl<'de> Deserialize<'de> for RadixTree {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct RadixTreeData {
            v4_entries: Vec<(String, IpCategory)>,
            v6_entries: Vec<(String, IpCategory)>,
            metadata: HashMap<String, String>,
            stats: LookupStats,
        }

        let data = RadixTreeData::deserialize(deserializer)?;
        let mut tree = RadixTree::default();
        
        // Rebuild the v4 table
        for (net_str, cat) in data.v4_entries {
            let net: IpNetwork = net_str.parse().map_err(serde::de::Error::custom)?;
            tree.v4_table.insert(net, cat);
        }
        
        // Rebuild the v6 table
        for (net_str, cat) in data.v6_entries {
            let net: IpNetwork = net_str.parse().map_err(serde::de::Error::custom)?;
            tree.v6_table.insert(net, cat);
        }
        
        tree.metadata = data.metadata;
        tree.stats = data.stats;
        
        Ok(tree)
    }
}

/// Statistics about lookups in the radix tree
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct LookupStats {
    pub total_lookups: u64,
    pub hits: u64,
    pub misses: u64,
    pub last_updated: Option<chrono::DateTime<chrono::Utc>>,
}

impl RadixTree {
    /// Create a new, empty RadixTree
    pub fn new() -> Self {
        Self {
            v4_table: IpNetworkTable::new(),
            v6_table: IpNetworkTable::new(),
            metadata: HashMap::new(),
            stats: LookupStats::default(),
        }
    }

    /// Insert an IP network with its category into the tree
    /// 
    /// Returns the previous category if the network was already in the tree, or None if it was a new entry.
    pub fn insert(&mut self, network: IpNetwork, category: IpCategory) -> Option<IpCategory> {
        match network {
            IpNetwork::V4(net) => self.v4_table.insert(net, category),
            IpNetwork::V6(net) => self.v6_table.insert(net, category),
        }
    }

    /// Remove an IP network from the tree
    pub fn remove(&mut self, network: IpNetwork) -> Option<IpCategory> {
        match network {
            IpNetwork::V4(net) => self.v4_table.remove(net),
            IpNetwork::V6(net) => self.v6_table.remove(net),
        }
    }

    /// Check if an IP address is in the tree and return its category if found
    pub fn lookup(&self, ip: IpAddr) -> Option<IpCategory> {
        let result = match ip {
            IpAddr::V4(ip) => self.v4_table.longest_match(ip).map(|(_, &t)| t),
            IpAddr::V6(ip) => self.v6_table.longest_match(ip).map(|(_, &t)| t),
        };

        // Update stats
        let mut stats = self.stats.clone();
        stats.total_lookups += 1;
        if result.is_some() {
            stats.hits += 1;
        } else {
            stats.misses += 1;
        }
        // Note: We're not updating stats here since we can't mutate self in a shared reference
        // The actual stats update happens in the SharedRadixTree wrapper
        
        result
    }

    /// Get the number of networks in the tree as a tuple (v4_count, v6_count)
    pub fn len(&self) -> (usize, usize) {
        (self.v4_table.len().0, self.v6_table.len().0)
    }

    /// Get the total number of networks in the tree (both IPv4 and IPv6)
    pub fn total_len(&self) -> usize {
        self.v4_table.len().0 + self.v6_table.len().0
    }

    /// Check if the tree is empty
    pub fn is_empty(&self) -> bool {
        self.v4_table.len().0 == 0 && self.v6_table.len().0 == 0
    }

    /// Load IP ranges into the tree
    pub fn load_ranges(&mut self, ranges: &[IpRange]) -> Result<()> {
        for range in ranges {
            if let Ok(network) = range.network.parse::<IpNetwork>() {
                // We don't care about the previous value when loading ranges,
                // so we can safely ignore the Option returned by insert
                self.insert(network, range.category);
            } else {
                return Err(IpRangeError::InvalidNetwork(range.network.clone()));
            }
        }
        self.stats.last_updated = Some(chrono::Utc::now());
        Ok(())
    }

    /// Get the current lookup statistics
    pub fn stats(&self) -> &LookupStats {
        &self.stats
    }

    /// Save the tree to a file
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let serialized = serde_json::to_vec_pretty(self)?;
        if let Some(dir) = path.as_ref().parent() {
            fs::create_dir_all(dir)?;
        }
        fs::write(path, serialized)?;
        Ok(())
    }

    /// Load a tree from a file
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let data = fs::read(path)?;
        let tree: Self = serde_json::from_slice(&data)?;
        Ok(tree)
    }
}

/// A thread-safe wrapper around RadixTree
#[derive(Debug, Clone)]
pub struct SharedRadixTree {
    inner: Arc<RwLock<RadixTree>>,
}

impl SharedRadixTree {
    /// Create a new, empty SharedRadixTree
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(RadixTree::new())),
        }
    }

    /// Lookup an IP address in the tree
    pub fn lookup(&self, ip: IpAddr) -> Option<IpCategory> {
        let mut tree = self.inner.write();
        let result = tree.lookup(ip);
        
        // Update stats
        if result.is_some() {
            tree.stats.hits += 1;
        } else {
            tree.stats.misses += 1;
        }
        tree.stats.total_lookups += 1;
        
        result
    }

    /// Replace the current tree with a new one
    pub fn replace(&self, new_tree: RadixTree) {
        *self.inner.write() = new_tree;
    }

    /// Get the current lookup statistics
    pub fn stats(&self) -> LookupStats {
        self.inner.read().stats.clone()
    }

    /// Get the number of networks in the tree
    pub fn len(&self) -> (usize, usize) {
        self.inner.read().len()
    }

    /// Get the total number of networks in the tree (both IPv4 and IPv6)
    pub fn total_len(&self) -> usize {
        self.inner.read().total_len()
    }

    /// Check if the tree is empty
    pub fn is_empty(&self) -> bool {
        self.inner.read().is_empty()
    }

    /// Save the tree to a file
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        self.inner.read().save_to_file(path)
    }

    /// Load a tree from a file
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let tree = RadixTree::load_from_file(path)?;
        Ok(Self {
            inner: Arc::new(RwLock::new(tree)),
        })
    }
}

impl Default for SharedRadixTree {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

    #[test]
    fn test_insert_and_lookup() {
        let mut tree = RadixTree::new();
        let network = IpNetwork::V4("192.168.1.0/24".parse().unwrap());
        tree.insert(network, IpCategory::Vpn);

        let ip = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 10));
        assert_eq!(tree.lookup(ip), Some(IpCategory::Vpn));
        
        let ip = IpAddr::V4(Ipv4Addr::new(192, 168, 2, 10));
        assert_eq!(tree.lookup(ip), None);
        
        // Test len()
        assert_eq!(tree.len(), (1, 0));
        assert_eq!(tree.total_len(), 1);
        assert!(!tree.is_empty());
    }

    #[test]
    fn test_shared_tree() {
        let tree = SharedRadixTree::new();
        let ip = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 10));
        
        // Initially empty
        assert_eq!(tree.lookup(ip), None);
        assert_eq!(tree.len(), (0, 0));
        assert!(tree.is_empty());
        
        // Create a new tree with some data
        let mut new_tree = RadixTree::new();
        let network = IpNetwork::V4("192.168.1.0/24".parse().unwrap());
        new_tree.insert(network, IpCategory::Vpn);
        
        // Replace the tree
        tree.replace(new_tree);
        
        // Now should find the IP
        assert_eq!(tree.lookup(ip), Some(IpCategory::Vpn));
        assert_eq!(tree.len(), (1, 0));
        assert_eq!(tree.total_len(), 1);
        assert!(!tree.is_empty());
        
        // Test stats
        let stats = tree.stats();
        assert_eq!(stats.total_lookups, 1);
        assert_eq!(stats.hits, 1);
        assert_eq!(stats.misses, 0);
    }
}
