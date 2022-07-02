use process_rust::objects::ocdg::exporter::export_ocdg;
use process_rust::objects::ocdg::{Ocdg, generate_ocdg, Relations};
use process_rust::objects::ocel::Ocel;
use process_rust::objects::ocel::importer::import_ocel;
use process_rust::objects::ocel::exporter::export_ocel_pretty;
use process_rust::objects::ocdg::importer::import_ocdg;
use std::time::Instant;
use petgraph::dot::Dot;

#[test]
fn test_ocdg_generation(){
    let import_time = Instant::now();
    // let log: Ocel = import_ocel("logs/min.jsonocel").unwrap();
    let log: Ocel = import_ocel("../ocel-features/examples/logs/actual-min-export.jsonocel").unwrap();
    let ocdg: Ocdg = import_ocdg("../../Desktop/test.gexf", &log).unwrap();

    // let log: Ocel = import_ocel("../../Downloads/p2p-rfc3339.jsonocel").unwrap();
    // let g = import_ocdg("../../Desktop/example.gexf").unwrap();
    // let g = import_ocdg("../../Desktop/example.gexf").unwrap();
    println!("Importing the OCEL took {:?}", import_time.elapsed());
    // let success = export_ocdg(&g, "../../Desktop/example-export.gexf");
    // let export_time = Instant::now();
    // let export_status = export_ocel_pretty(&log, "../ocel-features/examples/logs/actual-min-export.jsonocel").unwrap();
    // println!("Exporting the OCEL took {:?} -> {}", export_time.elapsed(), export_status);
    // println!("{:?}", &log.objects);
    // println!("{:?}", &log.events);
    // let log: Ocel = import_ocel("logs/min.jsonocel").unwrap();

    // let relations: Vec<Relations> = vec![Relations::DESCENDANTS]; 
    // let relations: Vec<Relations> = vec![Relations::INTERACTS, 
    //                                      Relations::DESCENDANTS,
    //                                      Relations::COBIRTH,
    //                                      Relations::COLIFE,
    //                                      Relations::CODEATH,
    //                                      Relations::CONSUMES,
    //                                      Relations::INHERITANCE,
    //                                      Relations::PEELER,
    //                                      Relations::ENGAGES,
    //                                      Relations::MINION,
    //                                      Relations::SPLIT,
    //                                      Relations::MERGE];

    // let ocdg_time = Instant::now();
    // let _net: Ocdg = generate_ocdg(&log, &relations);
    // println!("Generating the OCDG took {:?}", ocdg_time.elapsed());


    // export_ocdg(&_net, &log, "../../Desktop/test.gexf").unwrap();
    println!("{:?}", ocdg.inodes);
    // println!("{:?}", log.objects);
    // println!("{:?}", ocdg.iedges);
    println!("{:?}", ocdg.irels);
    println!("{:?}", Dot::new(&ocdg.net));

    assert!(true)
}
