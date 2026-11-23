//! Print a JSON account record. The terminal prompt keeps passwords out of shell history.
use conctr_auth::{Account, Role};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    let [username, role] = args.as_slice() else {
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
    let account = Account::new(username.clone(), role, &password)?;
    println!("{}", serde_json::to_string_pretty(&[account])?);
    Ok(())
}
