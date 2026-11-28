//! Print a JSON account record. The terminal prompt keeps passwords out of shell history.
use conctr_auth::{Account, Role};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);
    let (Some(username), Some(role), None) = (args.next(), args.next(), args.next()) else {
        return Err("Usage: conctr-account USERNAME admin|member".into());
    };
    let role = match role.as_str() {
        "admin" => Role::Admin,
        "member" => Role::Member,
        _ => return Err("Choose admin or member.".into()),
    };
    let password = rpassword::prompt_password("New password: ")?;
    if rpassword::prompt_password("Confirm password: ")? != password {
        return Err("Passwords differ.".into());
    }
    let account = Account::new(username, role, &password)?;
    println!("{}", serde_json::to_string_pretty(&[account])?);
    Ok(())
}
