
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename="gexf")]
pub struct Gexf {
    xmlns: String,
    #[serde(alias="xmlns:xsi", rename(serialize="xmlns:xsi"))]
    xmlnsxsi: String,
    #[serde(alias="xsi:schemaLocation", rename(serialize="xsi:schemaLocation"))]
    schemaloc: String,
    version: String,
    pub meta: Meta,
    pub graph: GraphGexf
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Meta {
    #[serde(rename = "$unflatten=creator")]
    pub creator: String,
    #[serde(rename = "$unflatten=description")]
    pub description: String
}


#[derive(Serialize, Deserialize, Debug)]
pub struct GraphGexf {
    defaultedgetype: String,
    pub attributes: Vec<AttributesGexf>,
    pub nodes: NodesGexf,
    pub edges: EdgesGexf
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AttributesGexf {
    pub class: String,
    #[serde(rename="attribute", default)]
    pub attributes: Vec<AttributeGexf>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AttributeGexf {
    pub id: String,
    pub title: String,
    #[serde(rename="type")]
    pub attr_type: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NodesGexf {
    #[serde(rename="node")]
    pub nodes: Vec<NodeGexf>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NodeGexf {
    pub id: String,
    pub label: String,
    pub attvalues: AttValuesGexf
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AttValuesGexf {
    #[serde(rename="attvalue")]
    pub attvalues: Vec<AttValueGexf>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AttValueGexf {
    #[serde(alias="for", rename(serialize="for"))]
    pub attr: String,
    pub value: String

}

#[derive(Serialize, Deserialize, Debug)]
pub struct EdgesGexf {
    #[serde(rename="edge")]
    pub edges: Vec<EdgeGexf>
}


#[derive(Serialize, Deserialize, Debug)]
pub struct EdgeGexf {
    pub source: String, 
    pub target: String,
    pub attvalues: AttValuesGexf
}
impl Gexf {
    pub(crate) fn new() -> Self {
        Self { xmlns: "http://gexf.net/1.3".to_owned(), 
               xmlnsxsi: "http://www.w3.org/2001/XMLSchema-instance".to_owned(), 
               schemaloc: "http://gexf.net/1.3 http://gexf.net/1.3/gexf.xsd".to_owned(), 
               version: "1.3".to_owned(), 
               meta: Meta::default(), 
               graph: GraphGexf::default() }
    }
}

impl Default for GraphGexf {
    fn default() -> Self {
        Self { defaultedgetype: "directed".to_owned(), 
               attributes: vec![], 
               nodes: NodesGexf { nodes: vec![] }, 
               edges: EdgesGexf { edges: vec![] } }
    }
}

impl Default for Meta {
    fn default() -> Self {
        Self { creator: format!("Made with {} version {} by {}.",env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"), env!("CARGO_PKG_AUTHORS")), description: "Object-Centric Directed Graph".to_owned() }
    }
}
