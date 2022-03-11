pub mod currency {
	use common_primitives::Balance;

	pub const TOKEN_DECIMALS: u32 = 18; // Change this if you use different decimals
	pub const TOKEN_SYMBOL: &str = "MAIN";
	pub const UNITS: Balance = 10_u128.pow(TOKEN_DECIMALS);
	pub const MILLICENTS: Balance = CENTS / 1000;
	pub const CENTS: Balance = DOLLARS / 100; // assume this is worth about a cent.
	pub const DOLLARS: Balance = UNITS;

	pub const fn deposit(items: u32, bytes: u32) -> Balance {
		items as Balance * 15 * CENTS + (bytes as Balance) * 6 * CENTS
	}
}
