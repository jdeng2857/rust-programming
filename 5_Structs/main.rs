struct User {
	active: bool,
	username: String,
	email: String,
	sign_in_count: u64,
}

fn main() {
	let mut user1 = User {
		active: true,
		username: String::from("Anakin"),
		email: String::from("anakin@skywalker.com"),
		sign_in_count: 12,
	};

	user1.email = String::from("darth@vader.com");

	let user2 = User {
		email: String::from("luke@skywalker.com"),
		..user1
	};
}

fn build_user(email: String, username: String) -> User {
	User {
		active: true,
		username,
		email,
		sign_in_count: 1,
	}
}