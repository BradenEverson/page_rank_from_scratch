//! Primary PageRank implementation that uses a page resgistry and constructs a stochastic travel
//! matrix based on results that match a search

use std::collections::HashMap;

use itertools::Itertools;
use slotmap::SlotMap;

use crate::{
    crawler::{SiteKey, SiteLog},
    graph_rank::ConnectionGraph,
};

/// Show the top {this number} results when searching for a topic
pub const RESULTS_TO_SHOW: usize = 250;

/// Struct responsible for creating stochastic matrices that represent sites that appear
pub struct PageRanker {
    /// The site registry
    sites: SlotMap<SiteKey, SiteLog>,
}

impl PageRanker {
    /// Creates a new PageRanker based on
    pub fn from_registry(sites: SlotMap<SiteKey, SiteLog>) -> Self {
        Self { sites }
    }

    pub fn search(&self, term: &str) -> Option<Vec<&SiteLog>> {
        let mut site_key_to_graph_keys = HashMap::new();
        let mut graph: ConnectionGraph<Option<SiteKey>> = ConnectionGraph::default();

        let within_term = self.reduce_registry_by_term(term);
        if within_term.len() == 0 {
            return None;
        }

        for site_key in &within_term {
            site_key_to_graph_keys.insert(site_key, graph.register());
            graph.set_val(site_key_to_graph_keys[&site_key], Some(*site_key));
        }

        for _ in 0..(RESULTS_TO_SHOW - within_term.len()) {
            let empty = graph.register();
            graph.set_val(empty, None);
            graph.connect(empty, empty, 1.0);
        }

        for (site_key, graph_key) in &site_key_to_graph_keys {
            let mut connections: Vec<_> = self.sites[**site_key]
                .connections
                .iter()
                .filter(|key| within_term.contains(key))
                .unique()
                .collect();
            if !connections.contains(site_key) {
                connections.push(site_key);
            }

            let prob = 1f32 / connections.len() as f32;

            for connection in connections {
                graph.connect(*graph_key, site_key_to_graph_keys[connection], prob);
            }
        }

        let rankings = graph.get_rankings::<RESULTS_TO_SHOW>()?;

        let top_sites: Vec<_> = rankings
            .into_iter()
            .filter_map(|key| graph.nodes[key].item)
            .map(|key| &self.sites[key])
            .collect();

        Some(top_sites)
    }

    /// Creates a reduced slotmap based on titles that match a search term
    fn reduce_registry_by_term(&self, term: &str) -> Vec<SiteKey> {
        let valid = self
            .sites
            .clone()
            .into_iter()
            .filter(|(_, site)| site.title.to_lowercase().contains(&term.to_lowercase()))
            .map(|(key, _)| key)
            .collect::<Vec<_>>();

        if valid.len() < RESULTS_TO_SHOW {
            valid
        } else {
            valid[..RESULTS_TO_SHOW].to_vec()
        }
    }
}
