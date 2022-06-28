use ocel::objects::ocdg::{Ocdg, generate_ocdg, Relations};
use ocel::objects::ocel::Ocel;
use ocel::objects::ocel::importer::import_ocel;
use std::time::Instant;

#[test]
fn test_ocdg_generation(){
    let import_time = Instant::now();
    // let log: Ocel = import_ocel("../ocel-features/examples/logs/actual-min.jsonocel").unwrap();
    let log: Ocel = import_ocel("../../Downloads/p2p-rfc3339.jsonocel").unwrap();
    println!("Importing the OCEL took {:?}", import_time.elapsed());
    // println!("{:?}", &log.objects);
    // println!("{:?}", &log.events);
    // let log: Ocel = import_ocel("logs/min.jsonocel").unwrap();
    let relations: Vec<Relations> = vec![Relations::INTERACTS, 
                                         Relations::DESCENDANTS,
                                         Relations::COBIRTH,
                                         Relations::COLIFE,
                                         Relations::CODEATH,
                                         Relations::CONSUMES,
                                         Relations::INHERITANCE,
                                         Relations::PEELER,
                                         Relations::ENGAGES,
                                         Relations::MINION,
                                         Relations::SPLIT,
                                         Relations::MERGE];

    let ocdg_time = Instant::now();
    let _net: Ocdg = generate_ocdg(&log, &relations);
    println!("Generating the OCDG took {:?}", ocdg_time.elapsed());
    // println!("{:?}", _net.inodes);
    // println!("{:?}", _net.iedges);
    // println!("{:?}", _net.irels);

    assert!(true)
}
