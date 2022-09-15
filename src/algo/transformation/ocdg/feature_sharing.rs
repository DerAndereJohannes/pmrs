
use std::collections::hash_map::Entry;

use ahash::{AHashMap, AHashSet};
use nohash_hasher::IntMap;
use petgraph::{Direction::Incoming, visit::Dfs};
use serde_json::Value;

use crate::objects::{ocel::Ocel, ocdg::{Relations, OcdgRelations, generate_ocdg, RelationError, Ocdg}, linker::link_objects};


pub fn generate_feature_sharing_map(log: &Ocel, prop_relations: Vec<Relations>, oids: Vec<usize>) -> Result<IntMap<usize, Option<AHashMap<String, Value>>>, RelationError> {

    // share properties between property relations defined by user
    let shared_property_values = share_property_values(log, prop_relations)?;
    
    // flow all properties per object type using ascendants
    Ok(flow_property_values(log, shared_property_values, &oids))
}

fn share_property_values(log: &Ocel, prop_relations: Vec<Relations>) -> Result<IntMap<usize, AHashMap<String, Value>>, RelationError> {
    // Return None if invalid property relations
    for rel in &prop_relations {
        if rel.is_directed() {
            return Err(RelationError);
        }
    }

    let property_graph: Ocdg = generate_ocdg(&log, &prop_relations);
    let log_property_link = link_objects(&log.object_map, &property_graph.object_map);

    let mut new_properties: IntMap<usize, AHashMap<String, Value>> = IntMap::default();

    // extract all omaps and shared features
    for (oid, oid_obj) in &log.objects {
        let mut oid_ovmap: AHashMap<String, Value> = oid_obj.ovmap.clone();
        let graph_oid = log_property_link.get_by_left(oid).expect("This cannot fail");
        let mut edges = property_graph.net.neighbors_directed(*property_graph.inodes.get(graph_oid).expect("this cannot fail"), Incoming);
        let mut ot_check: AHashMap<String, Vec<&usize>> = AHashMap::default();

        while let Some(node) = edges.next() {
            let curr_log_oid = log_property_link.get_by_right(&property_graph.net[node]).expect("This cannot fail");
            ot_check.entry(property_graph.node_attributes[&property_graph.net[node]].node_type.clone()).or_insert(vec![]).push(curr_log_oid);
        }


        for (ot, neighs) in ot_check.iter() {
            let mut value_holders: AHashMap<String, Value> = AHashMap::default();
            let mut to_remove: AHashSet<&str> = AHashSet::default();
            for n in neighs {
                let neigh_obj = &log.objects.get(n).expect("this cannot fail");
                if ot.as_str() != oid_obj.obj_type.as_str() {
                    for (key, value) in &neigh_obj.ovmap {
                        match value_holders.entry(key.to_owned()) {
                            Entry::Vacant(ent) => {ent.insert(value.to_owned());},
                            Entry::Occupied(ent) => {
                                if ent.get() != value {
                                    to_remove.insert(key);
                                }
                            }
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
        new_properties.entry(*oid).or_insert(oid_ovmap);
    }

    Ok(new_properties)
} 

fn flow_property_values(log: &Ocel, new_properties: IntMap<usize, AHashMap<String, Value>>, oids: &Vec<usize>) -> IntMap<usize, Option<AHashMap<String, Value>>> {
    let tc_relations = vec![Relations::ASCENDANTS]; // support only ascendants for now
    let tc_graph = generate_ocdg(&log, &tc_relations);
    let log_tc_link = link_objects(&log.object_map, &tc_graph.object_map);
    let mut flowed_properties: IntMap<usize, Option<AHashMap<String, Value>>> = IntMap::default();
    
    for oid in oids {
        if let Some(oid_obj) = &log.objects.get(oid) {
            let graph_oid = log_tc_link.get_by_left(oid).expect("This cannot fail");
            let mut edges = Dfs::new(&tc_graph.net, *tc_graph.inodes.get(graph_oid).expect("this cannot fail"));
            let mut ot_check: AHashMap<String, Vec<&usize>> = AHashMap::default();

            while let Some(node) = edges.next(&tc_graph.net) {
                let curr_log_oid = log_tc_link.get_by_right(&tc_graph.net[node]).expect("This cannot fail");
                ot_check.entry(tc_graph.node_attributes[&tc_graph.net[node]].node_type.clone()).or_insert(vec![]).push(curr_log_oid);
            }

            let mut oid_ovmap: AHashMap<String, Value> = oid_obj.ovmap.clone();

            for (ot, neighs) in ot_check.iter() { 
                let mut value_holders: AHashMap<String, Value> = AHashMap::default();
                let mut to_remove: AHashSet<&str> = AHashSet::default();

                for n in neighs {
                    if let Some(neigh_obj) = new_properties.get(n) {
                        if ot.as_str() != oid_obj.obj_type.as_str() {
                            for (key, value) in neigh_obj {
                                match value_holders.entry(key.to_owned()) {
                                    Entry::Vacant(ent) => {ent.insert(value.to_owned());},
                                    Entry::Occupied(ent) => {
                                        if ent.get() != value {
                                            to_remove.insert(key);
                                        }
                                    }
                                }
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
            flowed_properties.entry(*oid).or_insert(Some(oid_ovmap));
        } else {
            flowed_properties.entry(*oid).or_insert(None);
        }
    }
    flowed_properties
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::objects::ocel::importer::import_ocel;

    lazy_static::lazy_static!{
        static ref OCEL: Ocel = import_ocel("logs/ocel-complex-test.jsonocel").expect("What did you do to the file?");
    }

    #[test]
    fn test_feature_sharing_invalid_input() {
        let expected = Err(RelationError);
        assert_eq!(generate_feature_sharing_map(&OCEL, vec![Relations::DESCENDANTS], vec![]), expected);
    }

    #[test]
    fn test_property_sharing_unique_property_share_success() {
        let property_relations: Vec<Relations> = vec![Relations::COBIRTH];
        let property_share = share_property_values(&OCEL, property_relations).expect("cannot fail");

        let shared_to_order_1 = OCEL.object_map.get_by_left("o2").expect("test file was altered");

        let values_order_1 = property_share.get(shared_to_order_1).expect("cannot fail");
        // price is unique
        assert!(values_order_1.contains_key("item:shared:price"));
        assert_eq!(values_order_1.get("item:shared:price").expect("cannot fail"), 50);
    }


    #[test]
    fn test_property_sharing_non_unique_ignore_success() {
        let property_relations: Vec<Relations> = vec![Relations::COBIRTH];
        let property_share = share_property_values(&OCEL, property_relations).expect("cannot fail");
        
        let shared_to_order_1 = OCEL.object_map.get_by_left("o2").expect("test file was altered");
        let values_order_1 = property_share.get(shared_to_order_1).expect("guaranteed");
        
        // name is not unique from items
        assert!(!values_order_1.contains_key("item:shared:name"));

    }

    #[test]
    fn test_property_sharing_unique_multi_share_success() {
        let oids: Vec<usize> = vec![
            *OCEL.object_map.get_by_left("i3").expect("test file was altered"),
            *OCEL.object_map.get_by_left("i4").expect("test file was altered"),
            *OCEL.object_map.get_by_left("i5").expect("test file was altered"),
        ];
        let property_relations: Vec<Relations> = vec![Relations::COBIRTH];
        let property_share = share_property_values(&OCEL, property_relations).expect("cannot fail");
        
        // check if o2 shared destination with i3, i4, i5
        for oid in &oids {
            let values_item = property_share.get(oid).expect("cannot fail");
            assert!(values_item.contains_key("order:shared:destination"));
            assert_eq!(values_item.get("order:shared:destination").expect("cannot fail"), 7);
        }
    }

    #[test]
    fn test_flow_sharing_1_step_share_success() {
        let focus: usize = *OCEL.object_map.get_by_left("p1").expect("test file was altered");
        let oids: Vec<usize> = vec![focus];
        
        // create raw value shared output
        let mut raw_properties = IntMap::<usize, AHashMap<String, Value>>::default();
        for (oid, oid_obj) in &OCEL.objects {
            raw_properties.entry(*oid).or_insert(oid_obj.ovmap.clone());
        }

        let flowed_prop = flow_property_values(&OCEL, raw_properties, &oids);

        let values_package = flowed_prop.get(&focus).expect("this cannot fail").as_ref().expect("this cannot fail");

        assert!(values_package.contains_key("item:shared:category"));
        assert_eq!(values_package.get("item:shared:category").expect("cannot fail"), "Electronics");
        
    }

    #[test]
    fn test_flow_sharing_2_step_share_success() {
        let focus: usize = *OCEL.object_map.get_by_left("r1").expect("test file was altered");
        let oids: Vec<usize> = vec![focus];
        
        // create raw value shared output
        let mut raw_properties = IntMap::<usize, AHashMap<String, Value>>::default();
        for (oid, oid_obj) in &OCEL.objects {
            raw_properties.entry(*oid).or_insert(oid_obj.ovmap.clone());
        }

        let flowed_prop = flow_property_values(&OCEL, raw_properties, &oids);

        let values_route = flowed_prop.get(&focus).expect("this cannot fail").as_ref().expect("this cannot fail");

        assert!(values_route.contains_key("package:shared:width"));
        assert!(values_route.contains_key("package:shared:weight"));
        assert!(values_route.contains_key("package:shared:height"));
        assert!(values_route.contains_key("item:shared:category"));

        assert_eq!(values_route.get("package:shared:width").expect("cannot fail"), 50);
        assert_eq!(values_route.get("package:shared:height").expect("cannot fail"), 50);
        assert_eq!(values_route.get("package:shared:weight").expect("cannot fail"), 15);
        assert_eq!(values_route.get("item:shared:category").expect("cannot fail"), "Electronics");
    }

    #[test]
    fn test_flow_sharing_1_step_ignore_success() {
        let focus: usize = *OCEL.object_map.get_by_left("p2").expect("test file was altered");
        let oids: Vec<usize> = vec![focus];
        
        // create raw value shared output
        let mut raw_properties = IntMap::<usize, AHashMap<String, Value>>::default();
        for (oid, oid_obj) in &OCEL.objects {
            raw_properties.entry(*oid).or_insert(oid_obj.ovmap.clone());
        }

        let flowed_prop = flow_property_values(&OCEL, raw_properties, &oids);

        let values_package = flowed_prop.get(&focus).expect("this cannot fail").as_ref().expect("this cannot fail");

        assert!(!values_package.contains_key("item:shared:category"));
        assert!(values_package.contains_key("item:shared:name"));
    }

    #[test]
    fn test_full_feature_share_flow_success() {
        let focus: usize = *OCEL.object_map.get_by_left("r1").expect("test file was altered");
        let oids: Vec<usize> = vec![focus];
        let property_relations: Vec<Relations> = vec![Relations::COBIRTH];

        let feature_sharing_values = generate_feature_sharing_map(&OCEL, property_relations, oids).unwrap();
        let values_focus = feature_sharing_values.get(&focus).expect("this cannot fail").as_ref().expect("this cannot fail");

        assert!(values_focus.contains_key("item:shared:category"));
        assert!(values_focus.contains_key("item:shared:order:shared:destination"));

        // order -> item -> package -> route (o -> i -> p -> r)
        assert_eq!(values_focus.get("item:shared:category").expect("cannot fail"), "Electronics"); // from item directly
        assert_eq!(values_focus.get("item:shared:order:shared:destination").expect("cannot fail"), 7); // from prop-shared order to item to route
    }

    #[test]
    fn test_full_feature_share_flow_fail_success() {
        let focus: usize = 99; // 99 does not exist in the log
        let oids: Vec<usize> = vec![focus];
        let property_relations: Vec<Relations> = vec![Relations::COBIRTH];

        let feature_sharing_values = generate_feature_sharing_map(&OCEL, property_relations, oids).unwrap();
        assert!(feature_sharing_values.get(&focus).expect("this should not fail").is_none());
    }
}
