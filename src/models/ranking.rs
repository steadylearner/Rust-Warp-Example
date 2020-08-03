use rusqlite::{params, Connection, Result, NO_PARAMS};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameRanking {
    pub email: String,
    pub total_prize: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameRankingList(pub Vec<GameRanking>);

impl GameRankingList {
    pub fn rank(conn: &Connection) -> Result<Vec<GameRanking>> {
        // Find the emails for game participants.
        let mut email_stmt = conn.prepare(
            "
            SELECT DISTINCT users.email
            FROM games
                INNER JOIN users ON users.id = games.user_id;
        ",
        )?;

        let email_results = email_stmt.query_map(NO_PARAMS, |row| {
            let email: String = row.get(0)?;
            Ok(email)
        })?;

        let mut emails = Vec::new();
        for e in email_results {
            emails.push(e?);
        }
        // println!("{:#?}", emails);

        let mut group_of_profits = Vec::new();
        for email in emails.iter() {
            // println!("{}", email);

            let mut profits_stmt = conn.prepare(
                "
                SELECT win, stake_amount, number_of_participants
                FROM games
                    INNER JOIN users ON users.id = games.user_id WHERE users.email = (?1);
            ",
            )?;

            let mut profits: Vec<f64> = Vec::new();

            // Should organize this.
            let profit_results = profits_stmt.query_map(params![&email], |row| {
                let win: i8 = row.get(0)?;
                let stake_amount: f64 = row.get(1)?;
                let number_of_participants: i64 = row.get(2)?;

                // Make this to a function.
                let profit = if win == 0 {
                    let loss = stake_amount * -1.0f64;
                    loss
                } else {
                    let rest = number_of_participants - 1;
                    let earning = stake_amount * rest as f64;
                    earning
                };

                // println!("{}", profit);
                profits.push(profit);

                Ok(profit)
            })?;
            for profit_result in profit_results {
                profit_result?;
                // println!("{:#?}", profit_result?);
            }

            group_of_profits.push(profits);
        }
        // println!("{:#?}", group_of_profits);

        let total_prizes: Vec<f64> = group_of_profits
            .into_iter()
            .map(|group_of_profit| {
                let total_prize: f64 = group_of_profit.iter().sum();
                total_prize
            })
            .collect();
        // println!("{:#?}", total_prizes);

        // zip emails and total_prizes and turn them to the GameRanking.

        let emails_with_total_prizes = emails.iter().zip(total_prizes.iter());
        // println!("{:#?}", emails_with_total_prizes);

        let mut game_ranking_list: Vec<GameRanking> = Vec::new();
        for (email, total_prize) in emails_with_total_prizes {
            let game_ranking = GameRanking {
                email: email.to_owned(),
                total_prize: total_prize.to_owned(),
            };
            game_ranking_list.push(game_ranking);
        }

        // https://rust-lang-nursery.github.io/rust-cookbook/algorithms/sorting.html
        // https://users.rust-lang.org/t/how-to-sort-a-vec-of-floats/2838
        // b,a  => DESC and a,b vice versa.
        game_ranking_list.sort_by(|b, a| a.total_prize.partial_cmp(&b.total_prize).unwrap());

        Ok(game_ranking_list)
    }
}

// let game_ranking_example = GameRanking {
//     email: "steady@learner.com".into(),
//     total_prize: 100000f64,
// };

// let game_ranking_list = vec!(game_ranking_example);

// Ok(game_ranking_list)
