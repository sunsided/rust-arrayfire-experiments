//! This program renders three separate instances of Conway's Game of Life
//! simultaneously as the R, G and B channels of an image.

use arrayfire::*;
use std::time::SystemTime;

/// Window dimensions.
mod window_size {
    pub const WIDTH: i32 = 512;
    pub const HEIGHT: i32 = 512;
}

/// Game dimensions.
mod game_size {
    pub const WIDTH: u64 = 256;
    pub const HEIGHT: u64 = 256;
    pub const CHANNELS: u64 = 3; // 1 for grayscale, 3 for RGB, 4 for weird RGB due to alpha
}

fn main() {
    set_device(0);
    info();

    let seed = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Duration since UNIX_EPOCH failed");
    set_seed(seed.as_secs());

    let win = Window::new(
        window_size::WIDTH,
        window_size::HEIGHT,
        "Game of Life".to_string(),
    );

    let mut conway = game::Game::new(game_size::HEIGHT, game_size::WIDTH, game_size::CHANNELS);

    // Game loop.
    while !win.is_closed() {
        let state = conway.update();
        win.draw_image(&state, None);
    }
}

/// Implementation of Conway's (default) Game of Life using kernel convolution.
///
/// # Explanation of the Algorithm
///
/// Each cell has two distinct states: live (`1`) or dead (`0`).
/// In the well-known version, there are four rules that determine the follow-up state:
///
/// 1. Any live cell with fewer than two live neighbours dies, as if caused by under-population.
/// 2. Any live cell with two or three live neighbours lives on to the next generation.
/// 3. Any live cell with more than three live neighbours dies, as if by overcrowding.
/// 4. Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
///
/// We're going to use a kernel convolution to determine the size of the 3x3 neighborhood.
///
/// Rephrasing the above conditions and assuming we know the current state of a cell
/// as well as the size of its neighborhood, we arrive at the following pseudocode:
///
/// 1. `if (cell == 1 && neighbours < 2)                       -> cell = 0 // under-population`
/// 2. `if (cell == 1 && (neighbours == 2 or neighbours == 3)) -> cell = 1 // stable environment`
/// 3. `if (cell == 1 && neighbours > 3)                       -> cell = 0 // overcrowding`
/// 4. `if (cell == 0 && neighbours == 3)                      -> cell = 1 // reproduction`
///
/// We can make some observations about these conditions, namely:
///
/// - A) If a cell has less than two neighbors, the result will always be `0`, regardless
///      of the cell's current value (which might be `0` already); this is due to rule 1.
/// - B) If the cell has exactly three neighbors, the result will always be `1`, regardless
///      of the cell's current value (which might be `1` already); this is due to rules 2 and 4.
/// - C) Since three neighbors always cause the result to be `1`, we only need to check for
///      exactly two neighbors to ensure a cell is still surviving (rule 2).
/// - D) A cell can only be born with exactly three neighbors (rule 4) and more than three neighbors
///      cause a cell to die (rule 3); as a result, more than three neighbors always cause the
///      result to be `0`, regardless of the cell's current value.
///
/// We thus have:
///
/// 1. `if (neighbours < 2)               -> cell = 0 // observation A`
/// 2. `if (cell == 1 && neighbours == 2) -> cell = 1 // observation C`
/// 3. `if (neighbours == 3)              -> cell = 1 // observation B`
/// 4. `if (neighbours > 3)               -> cell = 0 // observation D`
///
/// As a result, we only need to compare against the values `2` and `3` to determine the next state.
/// Only looking at the neighbor conditions above (ignoring current cell state), we find that
///
/// ```pseudocode
/// (neighbours <  2) || (neighbours >  3)    -> cell = 0
/// (neighbours == 2) && (neighbours == 3)    -> cell = 1
/// ```
///
/// However,
///
/// ```pseudocode
/// (neighbours <  2) || (neighbours >  3) <=> !( neighbours == 2 || neighbours == 3 )
/// ```
///
/// Therefore, conditions 2. and 3. are (almost) sufficient to determine all four outcomes.
/// The missing piece is the current state of the cell:
///
/// - As we know from observation B / condition 3, a cell will always exist if there are exactly
///   three neighbors. We'll call this the `must_exist` condition; it is additive.
/// - From observation C / condition 2, a cell only continues to exist if it existed before.
///   We'll call this the `can_exist` condition; it is multiplicative.
///
/// Mathematically, we can now express the resulting state as a multiplication and addition:
///
/// ```pseudocode
/// next_state = current_state * can_exist + must_exist
/// ```
///
/// This is implemented in [Game::update()](struct.Game.html#method.update).
///
/// For the original article on the implementation, see Shehzan Mohammed's post
/// [Conway's Game of Life using ArrayFire](https://arrayfire.com/conways-game-of-life-using-arrayfire/).
mod game {
    use arrayfire::*;

    /// The `Game` struct holds game state for Conway's Game of Life.
    pub struct Game {
        neighborhood_kernel: Array<f32>,
        value_2: Array<f32>,
        value_3: Array<f32>,
        state: Array<f32>,
    }

    /// Implementation of Conway's Game of Life as outlined in the module description.
    impl Game {
        /// Creates a new instance of the `Conway` struct.
        pub fn new(height: u64, width: u64, channels: u64) -> Self {
            // Constant values. Note that `Dim4` is the dimension type available; values are [H, W, C, 1].
            let kernel = Self::build_3x3_neighborhood_size_kernel();
            let value_2 = constant::<f32>(2.0, Dim4::new(&[1, 1, 1, 1])); // the value `2`
            let value_3 = constant::<f32>(3.0, Dim4::new(&[1, 1, 1, 1])); // the value `3`

            // Initial state.
            let state = Self::create_state(height, width, channels);

            Self {
                neighborhood_kernel: kernel,
                value_2,
                value_3,
                state,
            }
        }

        /// Updates the current state to the next state.
        pub fn update(&mut self) -> &Array<f32> {
            let n_size = self.calculate_neighborhood_size();
            let can_exist = eq(&n_size, &self.value_2, false);
            let must_exist = eq(&n_size, &self.value_3, false);
            self.state = &self.state * can_exist + must_exist;
            &self.state
        }

        /// Determines the neighborhood size using the state obtained from
        /// `create_state` and the kernel obtained from `build_3x3_neighborhood_size_kernel`.
        fn calculate_neighborhood_size(&self) -> Array<f32> {
            convolve2(
                &self.state,
                &self.neighborhood_kernel,
                ConvMode::DEFAULT,
                ConvDomain::SPATIAL,
            )
        }

        /// Builds a kernel to determine the size of a cell's neighborhood.
        fn build_3x3_neighborhood_size_kernel() -> Array<f32> {
            // Since the value of the a cell can be represented as 1 (live) and 0 (dead), using convolution
            // will give us the number of neighbors of any cell.
            // The center value of the kernel is 0 as we do not want to count the cell itself.
            const KERNEL: [f32; 9] = [1.0, 1.0, 1.0, 1.0, 0.0, 1.0, 1.0, 1.0, 1.0];
            Array::new(&KERNEL, Dim4::new(&[3, 3, 1, 1]))
        }

        /// Creates the initial state by binarizing a uniform distribution.
        /// The resulting array is of shape (height, width, colors, ??)
        fn create_state(height: u64, width: u64, channels: u64) -> Array<f32> {
            let dims = Dim4::new(&[height, width, channels, 1]);
            let random_state = randu::<f32>(dims);
            Self::binarize_state(random_state).cast::<f32>()
        }

        /// Takes a random floating-point state and applies a threshold to binarize it.
        fn binarize_state(state: Array<f32>) -> Array<bool> {
            let threshold = constant::<f32>(0.5, Dim4::new(&[1, 1, 1, 1]));
            gt(&state, &threshold, false)
        }
    }
}
