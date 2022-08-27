use crate::objects::ocdg::{Ocdg, Relations};
use ahash::{AHashSet, AHashMap};
use nohash_hasher::IntSet;



pub fn general_object_split_in_place(mut ocdg: Ocdg) -> Ocdg {
    let mut ot_set = AHashSet::<String>::default();
    let mut to_remove: Vec<(usize, usize)> = vec![];
    ocdg.node_attributes.iter().for_each(|(_, v)| {ot_set.insert(v.node_type.clone());});
    for (src, tar_map) in &ocdg.irels {
        let mut ot_check = AHashMap::<String, IntSet<usize>>::default();
        let mut edge_list = AHashMap::<String, Vec<usize>>::default();
        for (tar, rel_map) in tar_map {
            let tar_type = &ocdg.node_attributes.get(tar).expect("this cannot fail").node_type;
            match rel_map.get(&Relations::DESCENDANTS.into()) {
                Some(v) => {
                    if !v.is_empty() {
                        ot_check.entry(tar_type.to_string()).or_default().extend(v);
                        edge_list.entry(tar_type.to_string()).or_default().push(*tar);
                    }
                },
                None => {}
            }
        }

        // check which edges need to be removed
        for (ot, rel_edges) in &edge_list {
            if ot_check.get(ot).expect("cannot fail").len() > 1 {
                rel_edges.iter()
                         .for_each(|edge_tar| {
                             to_remove.push((*src, *edge_tar));
                         });
            }
        }
    }

    // remove the edges -> make changes as we are no longer in the borrow
    for (src, tar) in to_remove {
        ocdg.node_attributes.entry(src).or_default().src_cut.insert(tar);
        ocdg.node_attributes.entry(tar).or_default().tar_cut.insert(src);
        ocdg.net.remove_edge(*ocdg.iedges.get(&src).expect("").get(&tar).expect(""));
        ocdg.iedges.get_mut(&src).expect("src no longer in iedge").remove(&tar);
        ocdg.irels.get_mut(&src).expect("src no longer in irel").remove(&tar);
    }

    ocdg
}



#[cfg(test)]
mod tests {
    use crate::objects::{ocel::importer::import_ocel, ocdg::{generate_ocdg, Relations}};
    use super::*;

    #[test]
    fn test_decompose_general_object_split() {
        let default: Ocdg = generate_ocdg(&import_ocel("logs/ocel-decomposition-test.jsonocel").expect("What did you do to the file?"), &vec![Relations::DESCENDANTS]);
        assert_eq!(default.net.edge_count(), 4);

        let decomposed: Ocdg = general_object_split_in_place(default);
        assert_eq!(decomposed.net.edge_count(), 2);

        let general_object = decomposed.object_map.get_by_left("i1").expect("cannot fail");
        assert_eq!(decomposed.node_attributes.get(general_object).unwrap().src_cut.len(), 2);
        assert_eq!(decomposed.node_attributes.get(general_object).unwrap().tar_cut.len(), 0);

        let child1 = decomposed.object_map.get_by_left("p1").expect("cannot fail");
        assert_eq!(decomposed.node_attributes.get(child1).unwrap().tar_cut.len(), 1);
        assert_eq!(decomposed.node_attributes.get(child1).unwrap().src_cut.len(), 0);

        let child2 = decomposed.object_map.get_by_left("p2").expect("cannot fail");
        assert_eq!(decomposed.node_attributes.get(child2).unwrap().tar_cut.len(), 1);
        assert_eq!(decomposed.node_attributes.get(child2).unwrap().src_cut.len(), 0);

        let nothing = decomposed.object_map.get_by_left("o1").expect("cannot fail");
        assert_eq!(decomposed.node_attributes.get(nothing).unwrap().src_cut.len(), 0);
        assert_eq!(decomposed.node_attributes.get(nothing).unwrap().tar_cut.len(), 0);
    }
}
