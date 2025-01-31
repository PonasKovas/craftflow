#![doc(
	html_favicon_url = "https://github.com/PonasKovas/craftflow/blob/master/assets/icon.png?raw=true"
)]
#![doc(
	html_logo_url = "https://github.com/PonasKovas/craftflow/blob/master/assets/icon.png?raw=true"
)]

mod builder;

use craftflow::CraftFlow;

craftflow::init!(ctx: CraftFlow);

/// A loader for Luau scripts as modules
pub struct LuauMods {}

impl LuauMods {}
