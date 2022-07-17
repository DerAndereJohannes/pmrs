use bimap::BiMap;

pub fn link_objects<'a>(map1: &'a BiMap<String, usize>, map2: &'a BiMap<String, usize>) -> BiMap<&'a usize, &'a usize> {

    let mut linker_map: BiMap<&'a usize, &'a usize> = BiMap::new();
    if map1.len() < map2.len() {
        for entity in map1 {
            if let Some(linked) = map2.get_by_left(entity.0) {
                linker_map.insert(entity.1, linked);
            }
        }
    } else {
        for entity in map2 {
            if let Some(linked) = map1.get_by_left(entity.0) {
                linker_map.insert(linked, entity.1);
            }
        }
    }
    linker_map
}
