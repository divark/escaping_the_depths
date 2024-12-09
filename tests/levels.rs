use std::str::FromStr;

use cucumber::{given, then, when, World};

use escaping_the_depths::levels::building::*;
use escaping_the_depths::puzzles::switch::*;

#[derive(Debug, Default, World)]
pub struct TestEnvironment {
    desired_width: usize,
    desired_height: usize,
    desired_depth: usize,

    level: Level<SwitchPuzzleTiles>,
}

#[given(regex = r"a level's width of ([0-9]+), height of ([0-9]+), and depth of ([0-9]+),")]
fn given_desired_dimensions(
    test_env: &mut TestEnvironment,
    width: usize,
    height: usize,
    depth: usize,
) {
    test_env.desired_width = width;
    test_env.desired_height = height;
    test_env.desired_depth = depth;
}

#[given(regex = r"a level with a width of ([0-9]+), height of ([0-9]+), and a depth of ([0-9]+),")]
fn given_generated_level(
    test_env: &mut TestEnvironment,
    width: usize,
    height: usize,
    depth: usize,
) {
    let dimensions = LevelDimensions::new(width, height, depth);
    test_env.level = Level::new(dimensions);
}

#[when("the level is created with the desired dimensions,")]
fn when_level_created_with_dimensions(test_env: &mut TestEnvironment) {
    let dimensions = LevelDimensions::new(
        test_env.desired_width,
        test_env.desired_height,
        test_env.desired_depth,
    );
    test_env.level = Level::new(dimensions);
}

#[when(regex = r"tile ([0-9]+), ([0-9]+), ([0-9]+) is painted as a land tile in the map,")]
fn when_tile_painted_on_level(test_env: &mut TestEnvironment, x: usize, y: usize, z: usize) {
    let grid_coordinate = GridCoordinates::new(x, y, z);
    test_env
        .level
        .paint(&grid_coordinate, SwitchPuzzleTiles::Blank);
}

#[then(regex = r"tile ([0-9]+), ([0-9]+), ([0-9]+) should be a ([a-zA-Z]+) tile.")]
fn verify_tile_is_type_of_tile(
    test_env: &mut TestEnvironment,
    x: usize,
    y: usize,
    z: usize,
    tile_type: String,
) {
    let tile_coordinates = GridCoordinates::new(x, y, z);

    let expected_tile_type = SwitchPuzzleTiles::from_str(&tile_type).unwrap();
    let actual_tile_type = test_env.level.get_tile(&tile_coordinates);
    assert_eq!(expected_tile_type, actual_tile_type);
}

fn main() {
    futures::executor::block_on(TestEnvironment::run("tests/features/levels.feature"));
}
