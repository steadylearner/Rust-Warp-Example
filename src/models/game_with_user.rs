use rusqlite::{
    // params,
    Connection,
    Result,
    NO_PARAMS,
};

use chrono::naive::NaiveDateTime;

// Profit Can be positive or negative. Because it is gamble. When you lose it will be negative. When you win it will be positive.
// Temporary and should imporve it.
#[derive(Debug)]
pub struct GameWithUser {
    pub email: String,
    pub win: i8,
    pub stake_amount: f64,
    pub number_of_participants: i64,
    pub created_at: NaiveDateTime,
    // pub profit: f64,
}

// impl GameWithUser {
//     pub fn profit(&self) {

//     }
// }

// Make it to function
// let profit = if win {
//     stake_amount * (number_of_participants - 1)
// } else {
//     stake_amount * -1f64
// };

#[derive(Debug)]
pub struct GameWithUserList(pub Vec<GameWithUser>);

impl GameWithUserList {
    pub fn list(conn: &Connection) -> Result<Vec<GameWithUser>> {
        let mut stmt = conn.prepare(
            "
            SELECT users.email, win, stake_amount, number_of_participants, created_at,
                FROM games
                    INNER JOIN users ON users.id = games.user_id
                        ORDER BY users.email;
        ",
        )?;

        let results = stmt.query_map(NO_PARAMS, |row| {
            let win: i8 = row.get(1)?;
            let stake_amount: f64 = row.get(2)?;
            // let number_of_participants: i64 = row.get(3)?;

            // let profit = if win == 0 {
            //     let loss = stake_amount * -1.0f64;
            //     loss
            // } else {
            //     let rest = number_of_participants -1;
            //     let earning = stake_amount * rest as f64;
            //     earning
            // };

            Ok(GameWithUser {
                email: row.get(0)?,
                win,
                stake_amount,
                number_of_participants: row.get(3)?,
                created_at: row.get(4)?,
                // profit,
            })
        })?;

        let mut game_results = Vec::new();
        for c in results {
            game_results.push(c?);
        }

        Ok(game_results)
    }
}
