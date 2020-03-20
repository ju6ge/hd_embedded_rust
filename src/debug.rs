#[macro_export]
macro_rules! debug {
	($($arg:tt)*) => {
		if cfg!(debug_assertions) {
			hprintln!($($arg)*).unwrap();
	}
	};
}
