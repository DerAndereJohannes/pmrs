use std::{error::Error, fs::File, io::Read, iter::FromIterator};
use nohash_hasher::{IntMap, IntSet};

use quick_xml::de::from_str;

use crate::objects::{ocdg::{variants::gexf::Gexf, Ocdg}, ocel::Ocel};

pub fn import_gexf_ocdg(file_path: &str) -> Result<Ocdg, Box<dyn Error>> {
   let mut s = String::new();
   File::open(file_path)?.read_to_string(&mut s)?;
   let g: Gexf = from_str(&s)?;

   let mut ocdg: Ocdg = Ocdg::default();

   for obj in &g.graph.nodes.nodes {
       let oid = obj.id.parse::<usize>()?;
       let new_node = ocdg.net.add_node(oid);
       ocdg.object_map.insert(obj.label.to_owned(), oid);

       ocdg.node_attributes.entry(oid).or_default().node_type = obj.attvalues.attvalues[0].value.to_owned();

       ocdg.inodes.entry(oid).or_insert(new_node);
   }

   // add src_cuts and tar_cuts after object map is complete
   for obj in g.graph.nodes.nodes {
       let oid = obj.id.parse::<usize>()?;
       let src_cut_prep = obj.attvalues.attvalues[1].value.replace("'", "\"");
       let src_cut_decode: Vec<&str> = ron::from_str(src_cut_prep.as_str())?;
       ocdg.node_attributes.entry(oid).or_default().src_cut = IntSet::from_iter(src_cut_decode.iter().map(|s| ocdg.object_map.get_by_left(*s).unwrap().to_owned()));
       let tar_cut_prep = obj.attvalues.attvalues[2].value.replace("'", "\"");
       let tar_cut_decode: Vec<&str> = ron::from_str(tar_cut_prep.as_str())?;
       ocdg.node_attributes.entry(oid).or_default().tar_cut = IntSet::from_iter(tar_cut_decode.iter().map(|s| ocdg.object_map.get_by_left(*s).unwrap().to_owned()));
   }

   let mut ev_id: usize = usize::MIN;
    for ev in g.graph.edges.edges {
       let src_o: usize = ev.source.parse::<usize>()?;
       let tar_o: usize = ev.target.parse::<usize>()?;

       for rel in ev.attvalues.attvalues {
           let rel_value_decode = rel.value.replace("'", "\"");
           let re: Vec<&str> = ron::from_str(rel_value_decode.as_str())?;
           ocdg.irels.entry(src_o).or_default()
                     .entry(tar_o).or_default()
                     .entry(rel.attr.parse::<u8>()?)
                     .or_insert(re.iter().map(|eid| {
                        match ocdg.event_map.get_by_left(*eid) {
                            Some(event_num) => {
                                *event_num
                            },
                            None => {
                                ocdg.event_map.insert(eid.to_string(), ev_id);
                                ev_id = ev_id + 1;
                                ev_id - 1
                            }
                        }
                     }).collect());
       }

       let new_edge = ocdg.net.add_edge(ocdg.inodes[&src_o], ocdg.inodes[&tar_o], 0);
       ocdg.iedges.entry(src_o).or_default().entry(tar_o).or_insert(new_edge);

   }

   Ok(ocdg)
}



pub fn import_gexf_ocdg_link_ocel(file_path: &str, log: &Ocel) -> Result<Ocdg, Box<dyn Error>> {
   let mut s = String::new();
   File::open(file_path)?.read_to_string(&mut s)?;
   let g: Gexf = from_str(&s)?;

   let mut ocdg: Ocdg = Ocdg::default();

   let file_to_log: IntMap<usize, &usize> = IntMap::from_iter(g.graph.nodes.nodes.iter()
                                                                                 .map(|node| (node.id.parse::<usize>().unwrap(), log.object_map.get_by_left(&node.label).unwrap())));


   for obj in g.graph.nodes.nodes {
       let oid = log.object_map.get_by_left(&obj.label).unwrap();
       let new_node = ocdg.net.add_node(*oid);

       ocdg.node_attributes.entry(*oid).or_default().node_type = obj.attvalues.attvalues[0].value.to_owned();

       ocdg.inodes.entry(*oid).or_insert(new_node);
   }

   for ev in g.graph.edges.edges {
       let src_o: &usize = file_to_log[&ev.source.parse::<usize>()?];
       let tar_o: &usize = file_to_log[&ev.target.parse::<usize>()?];

       for rel in ev.attvalues.attvalues {
           let re: Vec<&str> = ron::from_str(&rel.value)?;
           ocdg.irels.entry(*src_o).or_default()
                     .entry(*tar_o).or_default()
                     .entry(rel.attr.parse::<u8>()?)
                     .or_insert(re.iter().map(|eid| log.event_map.get_by_left(*eid).unwrap().to_owned()).collect());
       }

       let new_edge = ocdg.net.add_edge(ocdg.inodes[src_o], ocdg.inodes[tar_o], 0);
       ocdg.iedges.entry(*src_o).or_default().entry(*tar_o).or_insert(new_edge);

   }

   Ok(ocdg)
}
