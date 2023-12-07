use std::collections::HashMap;

use std::str::FromStr;

fn main() {
    let input = include_str!("./input.txt");

    let answer = find_lowest_location(input);

    println!("{answer}");
}

fn find_lowest_location(input: &str) -> usize {
    let input_chunks: Vec<String> = input.split("\n\n")
        .map(|s|s.to_string())
        .collect();
    let seeds: Vec<usize> =  input_chunks.get(0)
        .expect("Seeds section not found")
        .replace("seeds: ", "")
        .split(' ')
        .map(|seed|seed.parse::<usize>().unwrap())
        .collect();

    let maps_section = &input_chunks[1..];

    let mappings: Mappings = create_mappings(maps_section);

    let mut lowest_location: usize = usize::MAX;
    for seed in seeds {
        match mappings.location_for(seed) {
            MapResult::Done(location) => {
                if location < lowest_location {
                    lowest_location = location;
                }
            }
            _ => panic!("!!!!"),
        }
    }

    lowest_location
}

fn create_mappings(map_sections: &[String]) -> Mappings {
    let mut mappings = HashMap::new();
    for section in map_sections {
        let garden_map = section.parse::<GardenMap>().unwrap();
        mappings.insert(garden_map.from_type.clone(), garden_map);
    }
    Mappings { mappings }
}

#[derive(Debug)]
struct Mappings {
    mappings: HashMap<GardenType, GardenMap>
}

impl Mappings {
    fn location_for(&self, seed: usize) -> MapResult {
        let start_type = GardenType::Seed;
        let final_map= self.recursive_map(MapResult::Next(start_type, seed));
        final_map
    }

    fn recursive_map(&self, map_result: MapResult) -> MapResult {
        return match map_result {
            MapResult::Done(num) => MapResult::Done(num),
            MapResult::Next(next_garden_type, value) => {
                let next_mapper = self.mappings.get(&next_garden_type).unwrap();
                return match next_mapper.dest_for(value) {
                    MapResult::Next(next_mapper, next_value) => self.recursive_map(MapResult::Next(next_mapper, next_value)),
                    MapResult::Done(value) => MapResult::Done(value)
                }
            }
        }
    }

}

#[derive(Debug)]
struct GardenMap {
    from_type: GardenType,
    to_type: GardenType,
    mappings: Vec<Mapping>,
}

impl GardenMap {
    fn dest_for(&self, num: usize) -> MapResult {
        for mapping in &self.mappings {
            let Some(mapped) = mapping.ranged_map(num) else { continue; };
            return if self.to_type == GardenType::Location {
                MapResult::Done(mapped)
            } else {
                MapResult::Next(self.to_type.clone(), mapped)
            }

        }
        return if self.to_type == GardenType::Location {
            MapResult::Done(num)
        } else {
            MapResult::Next(self.to_type.clone(), num)
        }
    }
}

#[derive(Debug)]
struct Mapping {
    source: usize,
    dest: usize,
    range: usize
}

impl Mapping {
    fn ranged_map(&self, num: usize) -> Option<usize> {
        return if (self.source..(self.source + self.range)).contains(&num) {
            let diff = num - self.source;
            Some(self.dest + diff)
        } else {
            None
        }

    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
enum GardenType {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

#[derive(Debug)]
enum MapResult {
    Next(GardenType, usize),
    Done(usize)
}

impl FromStr for GardenType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "seed" => Ok(GardenType::Seed),
            "soil" => Ok(GardenType::Soil),
            "fertilizer" => Ok(GardenType::Fertilizer),
            "water" => Ok(GardenType::Water),
            "light" => Ok(GardenType::Light),
            "temperature" => Ok(GardenType::Temperature),
            "humidity" => Ok(GardenType::Humidity),
            "location" => Ok(GardenType::Location),
            _ => Err(String::from(format!("Could not create garden type {s}")))
        }
    }
}

impl FromStr for GardenMap {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().collect();
        let binding = lines[0].replace("-to-", " ");
        let header: Vec<&str> = binding.split(' ').collect();
        let from_type = &header[0];
        let to_type = &header[1];

        let mut mappings: Vec<Mapping> = Vec::new();
        for line in &lines[1..] {
            // dest source length
            let numbers: Vec<&str> = line.split(' ').collect();
            let dest = numbers[0].parse::<usize>()
                .expect("Could not unwrap dest");
            let source = numbers[1].parse::<usize>()
                .expect("Could not unwrap soruce");
            let length = numbers[2].parse::<usize>()
                .expect("Could not unwrap length");

            mappings.push(Mapping { source, dest, range: length })
        }

        let from_type: GardenType = from_type.parse().unwrap();
        let to_type: GardenType = to_type.parse().unwrap();
        Ok(GardenMap { from_type, to_type, mappings })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

        let answer = find_lowest_location(input);

        assert_eq!(35, answer);
    }

    #[test]
    fn mapping_test() {
        let mapping = Mapping { source: 98, dest: 50, range: 2 };

        let seed = 98;

        assert_eq!(50usize, mapping.ranged_map(seed).unwrap())
    }
}