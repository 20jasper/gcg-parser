use gcg_parser::{events::Coordinate, Gcg};

fn main() {
	let lines = [
		"#player1 20jasper Jasper",
		"#player2 xXFerrisXx Ferris The Crab",
		"#description 20jasper vs xXFerrisXx",
	];
	let text = lines.join("\n");

	let gcg = Gcg::build(&text).unwrap();

	println!("{gcg:?}");

	Coordinate::build("a1").unwrap();
}
