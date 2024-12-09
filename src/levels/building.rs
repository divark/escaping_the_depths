#[derive(Default, Debug)]
pub struct LevelDimensions {
    width: usize,
    height: usize,
    depth: usize,
}

impl LevelDimensions {
    pub fn new(width: usize, height: usize, depth: usize) -> Self {
        Self {
            width,
            height,
            depth,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn depth(&self) -> usize {
        self.depth
    }

    /// Returns a one-dimensional index flattened from a three-dimensional one
    /// provided.
    pub fn get_1d_idx(&self, grid_coordinate: &GridCoordinates) -> usize {
        let level_width = self.width();
        let level_height = self.height();
        let level_area = level_width * level_height;

        let tile_idx = (grid_coordinate.z() * level_area)
            + (grid_coordinate.x() * level_width)
            + grid_coordinate.y();
        tile_idx
    }
}

#[derive(Default, Debug)]
pub struct GridCoordinates {
    x: usize,
    y: usize,
    z: usize,
}

impl GridCoordinates {
    pub fn new(x: usize, y: usize, z: usize) -> Self {
        Self { x, y, z }
    }

    pub fn x(&self) -> usize {
        self.x
    }

    pub fn y(&self) -> usize {
        self.y
    }

    pub fn z(&self) -> usize {
        self.z
    }
}

#[derive(Default, Debug)]
pub struct Level<U>
where
    U: Default + Copy,
{
    tiles: Vec<U>,
    dimensions: LevelDimensions,
}

impl<U> Level<U>
where
    U: Default + Copy,
{
    pub fn new(dimensions: LevelDimensions) -> Self {
        let num_tiles = dimensions.width() * dimensions.height() * dimensions.depth();
        let tiles = vec![U::default(); num_tiles];
        Self { tiles, dimensions }
    }

    pub fn get_tile(&self, grid_coordinate: &GridCoordinates) -> U {
        let tile_idx = self.dimensions.get_1d_idx(grid_coordinate);
        let found_tile = self.tiles[tile_idx];
        found_tile
    }

    pub fn paint(&mut self, grid_coordinate: &GridCoordinates, paint_tile: U) {
        let tile_idx = self.dimensions.get_1d_idx(grid_coordinate);
        self.tiles[tile_idx] = paint_tile;
    }
}
