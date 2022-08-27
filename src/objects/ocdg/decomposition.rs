pub(self) mod general_object_split;

use self::general_object_split::general_object_split_in_place;

use super::Ocdg;


pub fn decompose_in_place(ocdg: Ocdg) -> Ocdg {
    general_object_split_in_place(ocdg)
}
