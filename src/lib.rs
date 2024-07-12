pub mod utils;
pub use utils::str_to_vecu8;
pub use utils::vecu8_to_str;
pub use utils::show;
pub use utils::same_row_index;
pub use utils::same_col_index;
pub use utils::same_block_index;

pub mod grid_task;
pub use grid_task::GridTask;
