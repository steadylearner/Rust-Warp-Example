// https://www.macmillandictionary.com/us/thesaurus-category/american/money-involved-in-gambling
// https://www.begambleaware.org/understanding-gambling/gambling-words-and-phrases-explained/

use rusqlite::{params, Connection, Result, NO_PARAMS};
use serde::{Deserialize, Serialize};

use chrono::naive::NaiveDateTime;

use crate::{
    utils::game::get_fair_odd,
};

use rand::Rng;

use log::{debug};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewGameRequest {
    pub stake_amount: Option<f64>,
    pub car_id: Option<i64>,
    pub number_of_participants: i64,
}

impl NewGameRequest {
    pub fn is_logically_valid(&self) -> bool {
        let bet_with_cash = self.stake_amount.is_some();
        let bet_with_car = self.car_id.is_some();

        let validity = if bet_with_cash && bet_with_car {
            false
        } else if &self.number_of_participants < &2i64 {
            false
        } else if &self.stake_amount.unwrap() <= &0f64 {
            // Logiaclly correct, but there should be better way.
            // Use this because of the type problem.
            false
        } else {
            true
        };
        validity
    }
}

pub fn find_game_result_and_profit(number_of_participants: i64, stake_amount: &f64) -> (bool, f64) {
    let odd = get_fair_odd(number_of_participants);

    let mut rng = rand::thread_rng();
    let from_zero_to_one = rng.gen::<f64>(); // [0, 1)
    let condition: bool = from_zero_to_one >= odd; // Include = because ) from_zero_to_one

    let win = if condition {
        debug!("You lost the gamble.");
        false
    } else {
        debug!("You won the gamble.");
        true
    };

    let profit = if !win {
        let loss = stake_amount * -1.0f64;
        loss
    } else {
        let rest = number_of_participants - 1;
        let earning = stake_amount * (rest as f64);
        earning
    };
    (win, profit)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewGame {
    pub stake_amount: f64,
    pub number_of_participants: i64,
    pub user_id: i64,
    pub win: bool, // 0(false, lost) or 1(true, won) temporarily because there is no boolean in SQLite.
}

impl NewGame {
    pub fn create(&self, conn: &Connection) -> Result<()> {
        // 0(false, lost) or 1(true, won) temporarily because there is no boolean in SQLite.
        let win = if self.win { 1 } else { 0 };

        conn.execute(
            "INSERT INTO
                games (stake_amount, number_of_participants, win, user_id)
                values (?1, ?2, ?3, ?4)
            ",
            &[
                &self.stake_amount.to_string(),
                &self.number_of_participants.to_string(),
                &win.to_string(),
                &self.user_id.to_string(),
            ],
        )?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct Game {
    pub id: i64,
    pub stake_amount: f64,
    pub number_of_participants: i64,
    // 0(false, lost) or 1(true, won) because there is no boolean in SQLite.
    pub win: i8,
    pub created_at: NaiveDateTime,
    pub user_id: i64,
}

impl Game {
    pub fn get(conn: &Connection, id: String) -> Result<Vec<Game>> {
        let mut stmt = conn.prepare("SELECT * FROM games WHERE id = (?1);")?;

        let result = stmt.query_map(params![&id], |row| {
            Ok(Game {
                id: row.get(0)?,
                stake_amount: row.get(1)?,
                number_of_participants: row.get(2)?,
                win: row.get(3)?,
                created_at: row.get(4)?,
                user_id: row.get(5)?,
            })
        })?;

        let mut game = Vec::new();
        for u in result {
            game.push(u?);
        }

        Ok(game)
    }
}

#[derive(Debug)]
pub struct GameList(pub Vec<Game>);

impl GameList {
    pub fn list(conn: &Connection) -> Result<Vec<Game>> {
        let mut stmt = conn.prepare("SELECT * FROM Games;")?;

        let result = stmt.query_map(NO_PARAMS, |row| {
            Ok(Game {
                id: row.get(0)?,
                stake_amount: row.get(1)?,
                number_of_participants: row.get(2)?,
                win: row.get(3)?,
                created_at: row.get(4)?,
                user_id: row.get(5)?,
            })
        })?;

        let mut games = Vec::new();
        for u in result {
            games.push(u?);
        }

        Ok(games)
    }
}
