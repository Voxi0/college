mod user;
use user::User;

fn main() -> Result<(), String> {
    let user: User = User::new()?;
    println!("");
    user.show();
    return Ok(());
}
