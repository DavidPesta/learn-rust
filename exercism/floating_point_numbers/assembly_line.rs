const CARS_PER_SPEED: f64 = 221.0;

pub fn production_rate_per_hour(speed: u8) -> f64 {
	match speed {
		0..=4 => CARS_PER_SPEED * (speed as f64) * 1.0,
		5..=8 => CARS_PER_SPEED * (speed as f64) * 0.9,
		9..=10 => CARS_PER_SPEED * (speed as f64) * 0.77,
		_ => panic!("Speed out of bound {}", speed)
	}
}

pub fn working_items_per_minute(speed: u8) -> u32 {
	(production_rate_per_hour(speed) / 60.0) as u32
}
