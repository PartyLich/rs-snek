use super::*;

/// A `Mapper` implementation backed by .csv file(s)
pub struct CsvMapper {}

impl Mapper for CsvMapper {
    fn load_map(&self) -> Result<WorldMap, Box<dyn Error>> {
        let mut map_list = get_csvs_from_dir("./resource");
        map_list.sort();
        let file_path = map_list.get(0).ok_or("No maps found in resource dir")?;
        let content = load_to_string(file_path.to_str().ok_or("PathBuf conversion err")?);
        let grid = csv_into_vec(&content);
        let walls = grid_to_position_vec(grid);

        Ok(WorldMap {
            color: types::WALL_COLOR,
            walls,
        })
    }
}
