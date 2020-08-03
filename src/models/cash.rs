use rusqlite::{Connection, Result};

// Should be transaction later.
pub fn update(conn: &Connection, amount: &f64, email: &str) -> Result<()> {
    let amount = amount.to_string();

    conn.execute(
        "UPDATE users SET cash = cash + (?1) WHERE email = (?2);",
        &[amount, email.to_owned()],
    )?;

    Ok(())
}
