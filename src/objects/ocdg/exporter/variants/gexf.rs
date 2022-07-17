use std::{fs::OpenOptions, io::{BufWriter, Write}, error::Error};
use quick_xml::se::to_string;
use strum::IntoEnumIterator;

use crate::objects::{ocdg::{variants::gexf::{Gexf, NodeGexf, AttValuesGexf, AttValueGexf, EdgeGexf, AttributesGexf, AttributeGexf}, Ocdg, Relations}, ocel::Ocel};



pub(crate) fn export_gexf_ocdg_string(g: &Ocdg, log: &Ocel, file_path: &str) -> Result<bool, Box<dyn Error>> {

    let mut gexf_repr: Gexf = Gexf::new();

    // object attr
    let mut node_attrs: Vec<AttributeGexf> = vec![];
    node_attrs.push(AttributeGexf { id: 0.to_string(), title: "type".to_string(), attr_type: "string".to_string()});
    node_attrs.push(AttributeGexf { id: 1.to_string(), title: "object_events".to_string(), attr_type: "liststring".to_string()});
    gexf_repr.graph.attributes.push(AttributesGexf { class: "node".to_string(), attributes: node_attrs });

    // edge attr
    let mut edge_attrs: Vec<AttributeGexf> = vec![];

    for rel in Relations::iter() {
        edge_attrs.push(AttributeGexf {id: rel.relation_index().to_string(), title: rel.to_string(), attr_type: "liststring".to_string()});
    }
    

    gexf_repr.graph.attributes.push(AttributesGexf { class: "edge".to_string(), attributes: edge_attrs });
    

    for (oid, data) in &g.node_attributes {
        let mut attrvalues: Vec<AttValueGexf> = vec![];
        attrvalues.push(AttValueGexf { attr: 0.to_string(), value: data.node_type.to_owned() });
        let oe_s: Vec<String> = log.objects.get(oid).unwrap().events.iter().map(|e_number| log.event_map.get_by_right(e_number).expect("This can't fail").to_owned()).collect();
        attrvalues.push(AttValueGexf { attr: 1.to_string(), value: format!("{:?}", oe_s) });

        gexf_repr.graph.nodes.nodes.push(NodeGexf {id: oid.to_string(), label: log.object_map.get_by_right(oid).expect("This can't fail").to_owned(), attvalues: AttValuesGexf {attvalues: attrvalues}});
    }

    for (src, edge_data) in &g.irels {
        for (tar, rels) in edge_data {
            let mut attrvalues: Vec<AttValueGexf> = vec![];
            for (r, events) in rels {
                let ev_s: Vec<String> = events.iter().map(|eid| log.event_map.get_by_right(eid).expect("This can't fail").to_owned()).collect();
                attrvalues.push(AttValueGexf { attr: r.to_string(), value: format!("{:?}", ev_s) });
            }

            gexf_repr.graph.edges.edges.push(EdgeGexf { source: src.to_string(), target: tar.to_string(), attvalues: AttValuesGexf { attvalues: attrvalues } });

        }
    }


    let mut ocdg_xml = r#"<?xml version="1.0" encoding="UTF-8"?>"#.to_string();
    ocdg_xml.push_str(&to_string(&gexf_repr).unwrap());

    let output_file = OpenOptions::new().create(true).write(true).truncate(true).open(file_path).unwrap();
    let mut f = BufWriter::new(output_file);
    f.write_all(ocdg_xml.as_bytes()).expect("Unable to write data");

    Ok(true)
}



pub(crate) fn export_gexf_ocdg(g: &Ocdg, file_path: &str) -> Result<bool, Box<dyn Error>> {

    let mut gexf_repr: Gexf = Gexf::new();

    // object attr
    let mut node_attrs: Vec<AttributeGexf> = vec![];
    node_attrs.push(AttributeGexf { id: 0.to_string(), title: "type".to_string(), attr_type: "string".to_string()});
    gexf_repr.graph.attributes.push(AttributesGexf { class: "node".to_string(), attributes: node_attrs });

    // edge attr
    let mut edge_attrs: Vec<AttributeGexf> = vec![];

    for rel in Relations::iter() {
        edge_attrs.push(AttributeGexf {id: rel.relation_index().to_string(), title: rel.to_string(), attr_type: "liststring".to_string()});
    }
    

    gexf_repr.graph.attributes.push(AttributesGexf { class: "edge".to_string(), attributes: edge_attrs });
    

    for (oid, data) in &g.node_attributes {
        let mut attrvalues: Vec<AttValueGexf> = vec![];
        attrvalues.push(AttValueGexf { attr: 0.to_string(), value: data.node_type.to_owned() });

        gexf_repr.graph.nodes.nodes.push(NodeGexf {id: oid.to_string(), label: g.object_map.get_by_right(oid).expect("This can't fail").to_owned(), attvalues: AttValuesGexf {attvalues: attrvalues}});
    }

    for (src, edge_data) in &g.irels {
        for (tar, rels) in edge_data {
            let mut attrvalues: Vec<AttValueGexf> = vec![];
            for (r, events) in rels {
                let ev_s: Vec<String> = events.iter().map(|eid| g.event_map.get_by_right(eid).expect("This can't fail").to_owned()).collect();
                attrvalues.push(AttValueGexf { attr: r.to_string(), value: format!("{:?}", ev_s) });
            }

            gexf_repr.graph.edges.edges.push(EdgeGexf { source: src.to_string(), target: tar.to_string(), attvalues: AttValuesGexf { attvalues: attrvalues } });

        }
    }


    let mut ocdg_xml = r#"<?xml version="1.0" encoding="UTF-8"?>"#.to_string();
    ocdg_xml.push_str(&to_string(&gexf_repr).unwrap());

    let output_file = OpenOptions::new().create(true).write(true).truncate(true).open(file_path).unwrap();
    let mut f = BufWriter::new(output_file);
    f.write_all(ocdg_xml.as_bytes()).expect("Unable to write data");

    Ok(true)
}

