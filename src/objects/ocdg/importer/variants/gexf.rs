use std::{error::Error, fs::File, io::Read};
use ahash::AHashMap;
use nohash_hasher::IntMap;

use quick_xml::de::from_str;

use crate::objects::{ocdg::{variants::gexf::Gexf, Ocdg}, ocel::Ocel};


pub fn import_gexf_ocdg(file_path: &str, log: &Ocel) -> Result<Ocdg, Box<dyn Error>> {
   let mut s = String::new();
   File::open(file_path).unwrap().read_to_string(&mut s).unwrap();
   let g: Gexf = from_str(&s)?;

   let mut ocdg: Ocdg = Ocdg::default();

   let obj_index: AHashMap<&str, &usize> = AHashMap::from_iter(log.objects.iter()
                                                                          .map(|(i, data)| (data.oid.as_str(), i)));
   let ev_index: AHashMap<&str, &usize> = AHashMap::from_iter(log.events.iter()
                                                                        .map(|(i, data)| (data.eid.as_str(), i)));

   let file_to_log: IntMap<usize, &usize> = IntMap::from_iter(g.graph.nodes.nodes.iter()
                                                                                 .map(|node| (node.id.parse::<usize>().unwrap(), obj_index[node.label.as_str()])));


   for obj in g.graph.nodes.nodes {
       let oid = obj_index[obj.label.as_str()];
       let new_node = ocdg.net.add_node(*oid);

       ocdg.node_attributes.entry(*oid).or_default().node_type = obj.attvalues.attvalues[0].value.to_owned();

       let oe: Vec<&str> = ron::from_str(&obj.attvalues.attvalues[1].value).unwrap();

       ocdg.node_attributes.entry(*oid).or_default().object_events = oe.iter().map(|eid| *ev_index[eid]).collect();
       ocdg.inodes.entry(*oid).or_insert(new_node);
   }

   for ev in g.graph.edges.edges {
       let src_o: &usize = file_to_log[&ev.source.parse::<usize>().unwrap()];
       let tar_o: &usize = file_to_log[&ev.target.parse::<usize>().unwrap()];

       for rel in ev.attvalues.attvalues {
           let re: Vec<&str> = ron::from_str(&rel.value).unwrap();
           ocdg.irels.entry(*src_o).or_default()
                     .entry(*tar_o).or_default()
                     .entry(rel.attr.parse::<usize>().unwrap())
                     .or_insert(re.iter().map(|eid| *ev_index[eid]).collect());
       }

       let new_edge = ocdg.net.add_edge(ocdg.inodes[src_o], ocdg.inodes[tar_o], 0);
       ocdg.iedges.entry(*src_o).or_default().entry(*tar_o).or_insert(new_edge);

   }

   Ok(ocdg)
}
