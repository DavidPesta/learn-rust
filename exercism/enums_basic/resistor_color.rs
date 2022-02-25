use int_enum::IntEnum;
use enum_iterator::IntoEnumIterator;

#[repr(u8)]
#[derive(Debug, PartialEq, Clone, Copy, IntEnum, IntoEnumIterator)]
pub enum ResistorColor {
	Black = 0,
	Brown = 1,
	Red = 2,
	Orange = 3,
	Yellow = 4,
	Green = 5,
	Blue = 6,
	Violet = 7,
	Grey = 8,
	White = 9,
}

pub fn color_to_value(_color: ResistorColor) -> u8 {
	_color.int_value()
}

pub fn value_to_color_string(value: u8) -> String {
	match ResistorColor::from_int(value) {
		Ok(color_enum) => format!("{:?}", color_enum),
		Err(_) => "value out of range".to_owned(),
	}
}

pub fn colors() -> Vec<ResistorColor> {
	ResistorColor::into_enum_iter().collect()
}
