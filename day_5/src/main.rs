use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use lazy_static::lazy_static;
use rayon::prelude::*;
use std::fs;
use std::ops::Range;
use std::str::FromStr;

lazy_static! {
    static ref PROGRESS_STYLE: ProgressStyle =
        ProgressStyle::with_template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>10}/{len:10}",)
            .unwrap()
            .progress_chars("##-");
}

#[derive(Debug, Eq, PartialEq)]
enum MapType {
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemp,
    TempToHumidity,
    HumidityToLocation,
}

#[derive(Debug, Eq, PartialEq)]
struct ParseMapTypeError;

impl FromStr for MapType {
    type Err = ParseMapTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let line = s.replace(':', "");
        let mut line_split = line.split_whitespace();

        let type_name = line_split.next();
        if type_name.is_none() {
            return Err(ParseMapTypeError);
        }

        match type_name.unwrap() {
            "seed-to-soil" => Ok(MapType::SeedToSoil),
            "soil-to-fertilizer" => Ok(MapType::SoilToFertilizer),
            "fertilizer-to-water" => Ok(MapType::FertilizerToWater),
            "water-to-light" => Ok(MapType::WaterToLight),
            "light-to-temperature" => Ok(MapType::LightToTemp),
            "temperature-to-humidity" => Ok(MapType::TempToHumidity),
            "humidity-to-location" => Ok(MapType::HumidityToLocation),
            _ => Err(ParseMapTypeError),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct AlmanacMapping {
    src_ranges: Vec<Range<u64>>,
    dest_ranges: Vec<Range<u64>>,
    map_type: MapType,
}

#[derive(Debug, Eq, PartialEq)]
struct ParseAlmanacMappingError;

impl Default for AlmanacMapping {
    fn default() -> Self {
        Self {
            src_ranges: vec![],
            dest_ranges: vec![],
            map_type: MapType::SeedToSoil,
        }
    }
}

impl FromStr for AlmanacMapping {
    type Err = ParseAlmanacMappingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        // Get mapping type
        let map_type = match lines.next() {
            Some(line) => match line.parse::<MapType>() {
                Ok(map_type) => Some(map_type),
                Err(_) => None,
            },
            None => None,
        };
        if map_type.is_none() {
            return Err(ParseAlmanacMappingError);
        }
        let map_type = map_type.unwrap();

        // Get ranges
        let mut src_ranges = vec![];
        let mut dest_ranges = vec![];
        for line in lines {
            let values: Vec<u64> = line
                .split_whitespace()
                .take(3)
                .filter_map(|num_str| num_str.parse::<u64>().ok())
                .collect();
            if values.len() != 3 {
                return Err(ParseAlmanacMappingError);
            }

            let dest_range = values[0]..(values[0] + values[2]);
            let src_range = values[1]..(values[1] + values[2]);

            src_ranges.push(src_range);
            dest_ranges.push(dest_range);
        }

        Ok(Self {
            map_type,
            src_ranges,
            dest_ranges,
        })
    }
}

impl AlmanacMapping {
    pub fn get_dest_for_src(&self, src: u64) -> u64 {
        for (i, src_range) in self.src_ranges.iter().enumerate() {
            if !src_range.contains(&src) {
                continue;
            }
            // Map src to dest
            let mut dest_range_copy = self.dest_ranges[i].clone();

            let range_idx = (src - src_range.start) as usize;
            return dest_range_copy.nth(range_idx).unwrap();
        }

        // Return unmapped value
        src
    }
}

#[derive(Debug, Eq, PartialEq, Default)]
struct Almanac {
    seeds: Vec<u64>,
    seed_to_soil: AlmanacMapping,
    soil_to_fertilizer: AlmanacMapping,
    fertilizer_to_water: AlmanacMapping,
    water_to_light: AlmanacMapping,
    light_to_temp: AlmanacMapping,
    temp_to_humidity: AlmanacMapping,
    humidity_to_location: AlmanacMapping,
}

#[derive(Debug, Eq, PartialEq)]
struct ParseAlmanacError;

impl Almanac {
    fn parse_seeds(line: &str) -> Result<Vec<u64>, ParseAlmanacError> {
        let mut line_split = line.split(": ");
        {
            let key = line_split.next();
            if key.is_none() || key.unwrap() != "seeds" {
                return Err(ParseAlmanacError);
            }
        }
        let seeds_str = line_split.next();
        if seeds_str.is_none() {
            return Err(ParseAlmanacError);
        }

        let seeds: Vec<u64> = seeds_str
            .unwrap()
            .split_whitespace()
            .filter_map(|seed_num| seed_num.parse::<u64>().ok())
            .collect();

        Ok(seeds)
    }

    fn set_almanac_mapping(&mut self, almanac_mapping: AlmanacMapping) {
        match almanac_mapping.map_type {
            MapType::SeedToSoil => self.seed_to_soil = almanac_mapping,
            MapType::SoilToFertilizer => self.soil_to_fertilizer = almanac_mapping,
            MapType::FertilizerToWater => self.fertilizer_to_water = almanac_mapping,
            MapType::WaterToLight => self.water_to_light = almanac_mapping,
            MapType::LightToTemp => self.light_to_temp = almanac_mapping,
            MapType::TempToHumidity => self.temp_to_humidity = almanac_mapping,
            MapType::HumidityToLocation => self.humidity_to_location = almanac_mapping,
        }
    }

    fn get_location_num(&self, seed: u64) -> u64 {
        let soil = self.seed_to_soil.get_dest_for_src(seed);
        let fertilizer = self.soil_to_fertilizer.get_dest_for_src(soil);
        let water = self.fertilizer_to_water.get_dest_for_src(fertilizer);
        let light = self.water_to_light.get_dest_for_src(water);
        let temp = self.light_to_temp.get_dest_for_src(light);
        let humidity = self.temp_to_humidity.get_dest_for_src(temp);
        self.humidity_to_location.get_dest_for_src(humidity)
    }

    pub fn get_seed_locations(&self) -> Vec<u64> {
        self.seeds
            .par_iter()
            .progress_with_style(PROGRESS_STYLE.clone())
            .map(|seed| self.get_location_num(*seed))
            .collect()
    }

    pub fn get_lowest_location(&self) -> Option<u64> {
        self.get_seed_locations().iter().min().map(|min| *min)
    }

    pub fn get_lowest_seed_range_location(&self) -> Option<u64> {
        if self.seeds.len() % 2 != 0 {
            return None;
        }
        let mut seed_ranges = self
            .seeds
            .iter()
            .enumerate()
            .step_by(2)
            .map(|(i, range_start)| {
                let range_size = self.seeds[i + 1];
                (*range_start)..(*range_start) + range_size
            })
            .collect::<Vec<Range<u64>>>();

        // Set up progress bar
        let pb = ProgressBar::new(sum_range_values(&seed_ranges));
        pb.set_style(PROGRESS_STYLE.clone());

        let location = seed_ranges
            .par_iter_mut()
            .progress_with_style(PROGRESS_STYLE.clone())
            .map(|range| {
                range.map(|src| {
                    let location = self.get_location_num(src);
                    pb.inc(1);
                    location
                })
            })
            .flatten_iter()
            .min();
        pb.finish();
        location
    }
}

impl FromStr for Almanac {
    type Err = ParseAlmanacError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        // Get Seeds
        let seed_line = lines.next();
        if seed_line.is_none() {
            return Err(ParseAlmanacError);
        }
        let seed_line = seed_line.unwrap();
        let seeds = Almanac::parse_seeds(seed_line)?;

        let mut almanac = Almanac::default();
        almanac.seeds = seeds;

        // Get AlmanacMappings
        let mut lines_buf = String::new();
        for line in lines.filter(|&line| !line.is_empty()) {
            // If line does not start with a digit, it's a new mapping
            if line.chars().next().unwrap().is_alphabetic() && !lines_buf.is_empty() {
                let almanac_mapping = lines_buf.parse::<AlmanacMapping>();
                if almanac_mapping.is_err() {
                    return Err(ParseAlmanacError);
                }
                lines_buf.clear();

                almanac.set_almanac_mapping(almanac_mapping.unwrap());
            }
            lines_buf.push_str(line);
            lines_buf.push('\n');
        }
        // Handle final mapping
        {
            let almanac_mapping = lines_buf.parse::<AlmanacMapping>();
            if almanac_mapping.is_err() {
                return Err(ParseAlmanacError);
            }
            lines_buf.clear();

            almanac.set_almanac_mapping(almanac_mapping.unwrap());
        }

        Ok(almanac)
    }
}

/// Combines two ranges into one if they overlap
fn combine_ranges<Idx: PartialOrd<Idx> + Copy>(
    range_0: &Range<Idx>,
    range_1: &Range<Idx>,
) -> Option<Range<Idx>> {
    if range_0.contains(&range_1.start) {
        Some(range_0.start..range_1.end)
    } else if range_1.contains(&range_0.start) {
        Some(range_1.start..range_0.end)
    } else {
        None
    }
}

/// Gets count of all values represented by the provided ranges
fn sum_range_values(ranges: &[Range<u64>]) -> u64 {
    ranges.iter().map(|range| &range.end - &range.start).sum()
}

fn main() {
    let almanac = fs::read_to_string("input.txt")
        .expect("failed to read input file")
        .parse::<Almanac>()
        .expect("failed to parse input file into Almanac data");

    let lowest_location = almanac.get_lowest_location();
    match lowest_location {
        Some(loc) => println!("Part 1 result: {loc}"),
        None => println!("Part 1 result not found"),
    }

    let lowest_range_location = almanac.get_lowest_seed_range_location();
    match lowest_range_location {
        Some(loc) => println!("Part 2 result: {loc}"),
        None => println!("Part 2 result not found"),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "seeds: 79 14 55 13\n\
                              \n\
                              seed-to-soil map:\n\
                              50 98 2\n\
                              52 50 48\n\
                              \n\
                              soil-to-fertilizer map:\n\
                              0 15 37\n\
                              37 52 2\n\
                              39 0 15\n\
                              \n\
                              fertilizer-to-water map:\n\
                              49 53 8\n\
                              0 11 42\n\
                              42 0 7\n\
                              57 7 4\n\
                              \n\
                              water-to-light map:\n\
                              88 18 7\n\
                              18 25 70\n\
                              \n\
                              light-to-temperature map:\n\
                              45 77 23\n\
                              81 45 19\n\
                              68 64 13\n\
                              \n\
                              temperature-to-humidity map:\n\
                              0 69 1\n\
                              1 0 69\n\
                              \n\
                              humidity-to-location map:\n\
                              60 56 37\n\
                              56 93 4";

    const SEEDS_LINE: &str = "seeds: 79 14 55 13";

    #[test]
    fn parse_map_type_from_str() {
        let test_data = [
            ("seed-to-soil map:", MapType::SeedToSoil),
            ("soil-to-fertilizer map:", MapType::SoilToFertilizer),
            ("fertilizer-to-water map:", MapType::FertilizerToWater),
            ("water-to-light map:", MapType::WaterToLight),
            ("light-to-temperature map:", MapType::LightToTemp),
            ("temperature-to-humidity map:", MapType::TempToHumidity),
            ("humidity-to-location", MapType::HumidityToLocation),
        ];

        for (line, expected) in test_data {
            assert_eq!(line.parse::<MapType>(), Ok(expected));
        }

        assert_eq!(SEEDS_LINE.parse::<MapType>(), Err(ParseMapTypeError));
    }

    #[test]
    fn parse_almanac_mapping_from_str() {
        let test_input = "water-to-light map:\n\
                                88 18 7\n\
                                18 25 70";
        let expected = AlmanacMapping {
            map_type: MapType::WaterToLight,
            src_ranges: vec![18..25, 25..95],
            dest_ranges: vec![88..95, 18..88],
        };

        assert_eq!(test_input.parse::<AlmanacMapping>(), Ok(expected));

        assert_eq!(
            SEEDS_LINE.parse::<AlmanacMapping>(),
            Err(ParseAlmanacMappingError)
        );
    }

    #[test]
    fn almanac_mapping_get_dest_for_src() {
        let mapping = AlmanacMapping {
            map_type: MapType::SeedToSoil,
            src_ranges: vec![98..100, 50..98],
            dest_ranges: vec![50..52, 52..100],
        };
        let test_data = [(79, 81), (14, 14), (55, 57), (13, 13), (98, 50)];

        for (src, dest) in test_data {
            assert_eq!(mapping.get_dest_for_src(src), dest);
        }
    }

    #[test]
    fn parse_seeds_line_from_str() {
        let expected = vec![79, 14, 55, 13];
        assert_eq!(Almanac::parse_seeds(SEEDS_LINE), Ok(expected));

        assert_eq!(
            Almanac::parse_seeds("sounds: 32 12 454"),
            Err(ParseAlmanacError)
        );
    }

    #[test]
    fn parse_almanac_from_str() {
        let almanac = TEST_INPUT.parse::<Almanac>();
        assert!(almanac.is_ok());
    }

    #[test]
    fn almanac_get_seed_locations() {
        let almanac = TEST_INPUT.parse::<Almanac>().unwrap();
        let expected = vec![82, 43, 86, 35];

        assert_eq!(almanac.get_seed_locations(), expected);
    }

    #[test]
    fn almanac_get_lowest_location() {
        let almanac = TEST_INPUT.parse::<Almanac>().unwrap();
        let expected = 35;

        assert_eq!(almanac.get_lowest_location(), Some(expected));
    }

    #[test]
    fn almanac_get_lowest_seed_range_location() {
        let almanac = TEST_INPUT.parse::<Almanac>().unwrap();
        let expected = 46;

        assert_eq!(almanac.get_lowest_seed_range_location(), Some(expected));
    }

    #[test]
    fn combine_ranges_test() {
        let test_data = [
            (0..10, 5..35, Some(0..35)),
            (11..86, 3..18, Some(3..86)),
            (25..100, 0..10, None),
        ];

        for (range_0, range_1, expected) in test_data {
            assert_eq!(combine_ranges(&range_0, &range_1), expected);
        }
    }

    #[test]
    fn sum_range_values_test() {
        let test_data = [(vec![0..10, 5..35], 40), (vec![11..86, 3..18], 90)];

        for (ranges, expected) in test_data {
            assert_eq!(sum_range_values(&ranges), expected);
        }
    }

    // #[test]
    // fn condense_ranges_test() {
    //     let test_data = [
    //         (vec![], vec![]),               // base case
    //         (vec![0..3], vec![0..3]),       // single item
    //         (vec![0..3, 1..5], vec![0..5]), // one merge
    //         (vec![0..3, 1..5, 10..20], vec![0..5, 10..20]),
    //         (vec![0..3, 1..5, 10..20, 15..18], vec![0..5, 10..20]),
    //     ];
    //
    //     for (input_ranges, expected) in test_data {
    //         assert_eq!(condense_ranges(&input_ranges), expected);
    //     }
    // }
}
