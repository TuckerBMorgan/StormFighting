mod animation;
mod character;
mod round;
mod game;
mod input;
mod collision;
mod projectile;
mod menu;
mod setup_functions;
mod character_sheet;

#[cfg(target_arch = "wasm32")]
mod web_net;

#[cfg(not(target_arch = "wasm32"))]
mod net;


pub use animation::*;
pub use character::*;
pub use round::*;
pub use game::*;
pub use collision::*;
pub use input::*;
pub use projectile::*;
pub use menu::*;
pub use setup_functions::*;
pub use character_sheet::*;

#[cfg(not(target_arch = "wasm32"))]
pub use net::*;


#[cfg(target_arch = "wasm32")]
pub use web_net::*;