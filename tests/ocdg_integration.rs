use std::collections::HashSet;
use std::iter::FromIterator;

use ahash::HashMap;
use pmrs::algo::transformation::ocel::situations::event_situations::{collect_event_targets, EventSituations, EventSituationParameters};
use pmrs::algo::transformation::ocel::situations::situation_sublog::{generate_event_situation_sublog, ExtractionPlan};
// use pmrs::objects::linker::link_objects;
// use pmrs::objects::ocdg::exporter::export_ocdg;
use pmrs::objects::ocdg::{generate_ocdg, Relations};
use pmrs::objects::ocel::Ocel;
use pmrs::objects::ocel::importer::import_ocel;
use polars::prelude::DataFrame;
// use pmrs::objects::ocel::exporter::export_ocel_pretty;
// use pmrs::objects::ocdg::importer::import_ocdg;
// use pmrs::algo::transformation::ocel::features::object_point::{ObjectPoint, ObjectPointConfig, object_point_features};
// use serde_json::Value;
// use std::collections::HashMap;
// use std::time::Instant;
// use std::thread;
// use std::time;
// use petgraph::dot::Dot;
// use pmrs::objects::ocdg::decomposition::decompose_in_place;
// use pmrs::objects::ocdg::exporter::export_ocdg;
// use pmrs::objects::ocdg::{generate_ocdg, Ocdg, Relations};
// use pmrs::objects::ocel::importer::import_ocel;

// #[test]
// fn test_ocdg_generation() {
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

    // let relations: Vec<Relations> = vec![Relations::DESCENDANTS, Relations::COBIRTH];

    // // let logs: Vec<&str> = vec!["0.5m.jsonocel", "1m.jsonocel", "2m.jsonocel", "3m.jsonocel", "6m.jsonocel", "12m.jsonocel", "24m.jsonocel"];
    // let log: Ocel = import_ocel("tests/test.jsonocel").unwrap();
    // // let log: Ocel = import_ocel("logs/ocel-complex-test.jsonocel").unwrap();
    // let ocdg = generate_ocdg(&log, &relations);

    // let mut event_config = EventSituationParameters::default();
    // event_config.activities = Some(HashSet::from_iter(vec!["fail delivery", "deliver package"]));

    // let situations = collect_event_targets(&log, EventSituations::EventChoice, event_config); 
    // println!("{:#?}", situations);

    // let ext_plan = ExtractionPlan::PrefixRoot; 
    // let df_vec: Vec<DataFrame> = Vec::with_capacity(situations.len());
    // // let mut features: HashMap<String, Vec<>>

    // for (eid, target) in situations {

    //     let situation_sublogs = generate_event_situation_sublog(&log, ext_plan, &eid);

    // }


    // // thread::sleep(time::Duration::from_secs(5));
    // // for log_path in logs {
    // //     let log: Ocel = import_ocel(format!("tests/{log_path}").as_str()).unwrap();
    // //     println!("Current log: {}, object #: {}, event #: {}", log_path, log.objects.len(), log.events.len());
    // //     let start = Instant::now();
    // //     let _ocdg = generate_ocdg(&log, &relations);
    //     let end = Instant::now();

    //     println!("OCDG generation took {:?}", end - start);
    //     thread::sleep(time::Duration::from_secs(5));
    // }

    // let relations: Vec<Relations> = vec![Relations::DESCENDANTS];
    // let default: Ocdg = generate_ocdg(&import_ocel("tests/test.jsonocel").expect("What did you do to the file?"), &relations);
    // export_ocdg(&default, "tests/th.gexf").unwrap();
    // let decomposed: Ocdg = decompose_in_place(default);
    // export_ocdg(&decomposed, "tests/descendants-decomposed.gexf").unwrap();
    
// }
//     // let import_time = Instant::now();
//     // let log: Ocel = import_ocel("logs/min.jsonocel").unwrap();
//     let log: Ocel = import_ocel("../ocel-features/examples/logs/actual-min.jsonocel").unwrap();
//     println!("{:?}", &log.events);
//     println!("{:?}", &log.events.get(&38).unwrap().timestamp > &log.events.get(&37).unwrap().timestamp);
//     // let ocdg: Ocdg = import_ocdg("../../Desktop/example-export.gexf").unwrap();
//     // println!("{:?}", &ocdg);

//     // let log: Ocel = import_ocel("../../Downloads/p2p-rfc3339.jsonocel").unwrap();
//     // let g = import_ocdg("../../Desktop/example.gexf").unwrap();
//     // let g = import_ocdg("../../Desktop/example.gexf").unwrap();
//     // println!("Importing the OCEL took {:?}", import_time.elapsed());
//     // let export_time = Instant::now();
//     // let export_status = export_ocel_pretty(&log, "../ocel-features/examples/logs/actual-min-export.jsonocel").unwrap();
//     // println!("Exporting the OCEL took {:?} -> {}", export_time.elapsed(), export_status);
//     // println!("{:?}", &log.objects);
//     // println!("{:?}", &log.events);
//     // let log: Ocel = import_ocel("logs/min.jsonocel").unwrap();

//     // let relations: Vec<Relations> = vec![Relations::DESCENDANTS]; 
//     // let relations: Vec<Relations> = vec![Relations::INTERACTS, 
//     //                                      Relations::DESCENDANTS,
//     //                                      Relations::COBIRTH,
//     // // let relations: Vec<Relations> = vec![Relations::COBIRTH,
//     //                                      Relations::COLIFE,
//     //                                      Relations::CODEATH,
//     //                                      Relations::CONSUMES,
//     //                                      Relations::INHERITANCE,
//     //                                      Relations::PEELER,
//     //                                      Relations::ENGAGES,
//     //                                      Relations::MINION,
//     //                                      Relations::SPLIT,
//     //                                      Relations::MERGE];

//     // let ocdg: Ocdg = generate_ocdg(&log, &relations);
//     // let params: HashMap<ObjectPoint, Option<Value>> = HashMap::from_iter([(ObjectPoint::ObjectLifetime, None), (ObjectPoint::ObjectEventInteractionOperator, None), (ObjectPoint::ObjectUnitSetRatio, None)]);

//     // let feature_config = ObjectPointConfig { ocel: &log, ocdg: &ocdg, params: &params};
//     // let feature_extraction = object_point_features(feature_config);
//     // println!("{:?}", feature_extraction);

//     // // let _success = export_ocdg(&ocdg, "../../Desktop/example-export.gexf");

//     // // export_ocdg(&_net, &log, "../../Desktop/test.gexf").unwrap();
//     // // println!("{:?}", ocdg.inodes);
//     // // println!("{:?}", log.objects);
//     // // println!("{:?}", ocdg.iedges);
//     // // println!("{:?}", ocdg.irels);
//     // // println!("{:?}", Dot::new(&ocdg.net));

//     assert!(true)
// }
