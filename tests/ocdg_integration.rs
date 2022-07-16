use pmrs::objects::linker::link_objects;
use pmrs::objects::ocdg::exporter::export_ocdg;
use pmrs::objects::ocdg::{Ocdg, generate_ocdg, Relations};
use pmrs::objects::ocel::Ocel;
use pmrs::objects::ocel::importer::import_ocel;
use pmrs::objects::ocel::exporter::export_ocel_pretty;
use pmrs::objects::ocdg::importer::import_ocdg;
use std::time::Instant;
use petgraph::dot::Dot;

#[test]
fn test_ocdg_generation(){
    let import_time = Instant::now();
    // let log: Ocel = import_ocel("logs/min.jsonocel").unwrap();
    let log: Ocel = import_ocel("../ocel-features/examples/logs/actual-min.jsonocel").unwrap();
    // println!("{:?}", &log.objects);
    // let ocdg: Ocdg = import_ocdg("../../Desktop/test.gexf", &log).unwrap();

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
    println!("{:?}", &log.object_map);
    println!("{:?}", &log.event_map);
    // let log: Ocel = import_ocel("logs/min.jsonocel").unwrap();

    // let relations: Vec<Relations> = vec![Relations::DESCENDANTS]; 
    let relations: Vec<Relations> = vec![Relations::INTERACTS, 
                                         Relations::DESCENDANTS,
                                         Relations::COBIRTH,
    // let relations: Vec<Relations> = vec![Relations::COBIRTH,
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
    let ocdg: Ocdg = generate_ocdg(&log, &relations);
    println!("Generating the OCDG took {:?}", ocdg_time.elapsed());

    let linkedo = link_objects(&log.object_map, &ocdg.object_map);
    let linkede = link_objects(&log.event_map, &ocdg.event_map);

    println!("{:?}", linkedo);
    println!("{:?}", linkede);


    // export_ocdg(&_net, &log, "../../Desktop/test.gexf").unwrap();
    // println!("{:?}", ocdg.inodes);
    // println!("{:?}", log.objects);
    // println!("{:?}", ocdg.iedges);
    // println!("{:?}", ocdg.irels);
    // println!("{:?}", Dot::new(&ocdg.net));

    assert!(true)
}
