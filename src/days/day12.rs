use crate::helpers::{map::Map, output::print_output, region::Region};
use std::{collections::HashMap, error::Error};

pub fn get_regions(map: &Map<String>) -> HashMap<String, Vec<Region>> {
    let mut regions_by_letter: HashMap<String, Vec<Region>> = HashMap::new();

    for cell in map {
        if !regions_by_letter.contains_key(cell.1) {
            regions_by_letter.insert(cell.1.clone(), Vec::new());
        }

        let regions_letter: Vec<Region> = regions_by_letter.remove(cell.1).unwrap();

        let mut new_regions_letter_vec: Vec<Region> = Vec::new();

        let mut new_cell_region = Region::new(cell.0.clone());

        for region in regions_letter {
            if region.distance_to_manhattan(&cell.0) <= 1 {
                new_cell_region.union(region);
            } else {
                new_regions_letter_vec.push(region);
            }
        }

        new_regions_letter_vec.push(new_cell_region);

        regions_by_letter.insert(cell.1.clone(), new_regions_letter_vec);
    }

    return regions_by_letter;
}

#[test]
pub fn day12a() -> Result<(), Box<dyn Error>> {
    let test_name = "day12a";
    let map: Map<String> = Map::read_input(test_name)?;

    let regions_letters: HashMap<String, Vec<Region>> = get_regions(&map);

    let mut sum = 0;
    for (_, regions_letter) in regions_letters {
        for region in regions_letter {
            sum += region.area() * region.perimeter();
        }
    }

    return print_output("day12a".to_string(), sum);
}

#[test]
pub fn day12b() -> Result<(), Box<dyn Error>> {
    let test_name = "day12a";
    let map: Map<String> = Map::read_input(test_name)?;

    let regions_letters: HashMap<String, Vec<Region>> = get_regions(&map);

    let mut sum = 0;
    for (letter, regions_letter) in regions_letters {
        for region in regions_letter {
            println!(
                "{:?} - s: {:?} p: {:?}",
                letter,
                region.area(),
                region.stright_perimeter()
            );
            sum += region.area() * region.stright_perimeter();
        }
    }

    return print_output("day12b".to_string(), sum);
}
