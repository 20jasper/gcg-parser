use gcg_parser::Player;
fn main() {
	let text = "#player1 20jasper";

	println!("{}", Player::build(text).unwrap_err());
}
