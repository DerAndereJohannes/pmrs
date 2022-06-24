use ocel::objects::ocdg::{Ocdg, generate_ocdg, Relations};
use ocel::objects::ocel::Ocel;
use ocel::objects::ocel::importer::import_ocel;
use petgraph::dot::Dot;

#[test]
fn test_ocdg_generation(){
    // let log: Ocel = import_ocel("../ocel-features/examples/logs/actual-min.jsonocel").unwrap();
    let log: Ocel = import_ocel("logs/min.jsonocel").unwrap();
    let relations: Vec<Relations> = vec![Relations::DESCENDANTS];
    let _net: Ocdg = generate_ocdg(&log, &relations);

    println!("{:?}", &Dot::new(&_net.net));

    assert!(true)
}
