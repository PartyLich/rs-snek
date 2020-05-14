use rand::Rng;

use super::*;

/// A `Mapper` implementation backed by .csv file(s)
pub struct CsvMapper {}

impl Mapper for CsvMapper {
    fn load_map(&self) -> Result<WorldMap, Box<dyn Error>> {
        let mut map_list = get_csvs_from_dir("./resource");
        map_list.sort();
        let map_selection = rand::thread_rng().gen_range(0, map_list.len());
        let file_path = map_list
            .get(map_selection)
            .ok_or("No maps found in resource dir")?;
        let content = load_to_string(file_path.to_str().ok_or("PathBuf conversion err")?);
        let grid = csv_into_vec(&content);
        let walls = grid_to_position_vec(grid);

        Ok(WorldMap {
            color: types::WALL_COLOR,
            walls,
        })
    }
}
