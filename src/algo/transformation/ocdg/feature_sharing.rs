
use std::collections::hash_map::Entry;

use ahash::{AHashMap, AHashSet};
use nohash_hasher::IntMap;
use petgraph::Direction::Incoming;
use serde_json::Value;

use crate::objects::{ocel::Ocel, ocdg::{Relations, OcdgRelations, generate_ocdg, RelationError, Ocdg}, linker::link_objects};


pub fn feature_sharing_map(log: &Ocel, tc_relations: Vec<Relations>, prop_relations: Vec<Relations>, oids: Vec<usize>) -> Result<IntMap<usize, Option<AHashMap<String, Value>>>, RelationError> {
   
    // Return None if invalid input time conscious relations
    for rel in &tc_relations {
        if !rel.is_timeconscious() {
            return Err(RelationError);
        }
    }

    // Return None if invalid property relations
    for rel in &prop_relations {
        if rel.is_directed() {
            return Err(RelationError);
        }
    }

    // generate property graph
    let property_graph: Ocdg = generate_ocdg(&log, &prop_relations);
    let log_property_link = link_objects(&log.object_map, &property_graph.object_map);

    let mut new_properties: IntMap<usize, Option<AHashMap<String, Value>>> = IntMap::default();

    // extract all omaps and shared features
    for oid in &oids {
        if let Some(oid_obj) = &log.objects.get(oid) {
            let graph_oid = log_property_link.get_by_left(oid).expect("This cannot fail");
            let mut edges = property_graph.net.neighbors_directed(*property_graph.inodes.get(graph_oid).expect("this cannot fail"), Incoming);
            let mut ot_check: AHashMap<String, Vec<&usize>> = AHashMap::default();
            while let Some(node) = edges.next() {
                let curr_log_oid = log_property_link.get_by_right(&property_graph.net[node]).expect("This cannot fail");
                ot_check.entry(property_graph.node_attributes[&property_graph.net[node]].node_type.clone()).or_insert(vec![]).push(curr_log_oid);
            }

            let mut oid_ovmap: AHashMap<String, Value> = oid_obj.ovmap.clone();

            for (ot, neighs) in ot_check.iter() {
                let mut value_holders: AHashMap<String, Value> = AHashMap::default();
                let mut to_remove: AHashSet<&str> = AHashSet::default();
                for n in neighs {
                    let oid_obj = &log.objects.get(n).expect("this cannot fail");
                    for (key, value) in &oid_obj.ovmap {
                        match value_holders.entry(key.to_owned()) {
                            Entry::Vacant(ent) => {ent.insert(value.to_owned());},
                            Entry::Occupied(ent) => {
                                if ent.get() != value {
                                    to_remove.insert(Value::as_str(value).expect("this should never fail"));
                                }
                            }
                        }
                    }
                    for (key, value) in value_holders.iter_mut() {
                        if !to_remove.contains(key.as_str()) {
                            oid_ovmap.entry(format!("{}:shared:{}", ot, key).to_string()).or_insert(value.to_owned());
                        }
                    }
                }
            }
            new_properties.entry(*oid).or_insert(Some(oid_ovmap));
        } else {
            new_properties.entry(*oid).or_insert(None);
        }
    }


    // generate time conscious graph
    let tc_graph = generate_ocdg(&log, &tc_relations);
    let log_tc_link = link_objects(&log.object_map, &tc_graph.object_map);


    Ok(new_properties)
}
