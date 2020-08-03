extern crate gamble;
use gamble::{
    db::sqlite::SQLITEPOOL,
    models::ranking::{GameRanking, GameRankingList},
};

extern crate rusqlite;
use rusqlite::Result;

#[macro_use]
extern crate prettytable;
use prettytable::Table;

fn main() -> Result<()> {
    let conn = SQLITEPOOL.get().unwrap();

    let mut table = Table::new();
    // let game_ranking_table_headers = format!(" Rank | Email | Total Prize");
    table.add_row(row![FY => "Rank", "Email", "Total Prize"]);

    let game_ranking_list = GameRankingList::rank(&conn)?;
    for (index, game_ranking) in game_ranking_list.into_iter().enumerate() {
        let rank = index + 1;
        let GameRanking { email, total_prize } = game_ranking;
        // let game_ranking_table_row = format!("{}, | {}, | {}", &rank, &email, &total_prize);
        // table.add_row(row![Fy->game_ranking_table_row]);
        // table.add_row(row![&rank, &email, &total_prize]);
        table.add_row(row![Fw => &rank, &email, &total_prize]);
    }

    table.printstd();

    Ok(())
}

// println!("{}. {}({})", rank, email, total_prize);

// | Rank |        email            |      Total Prize      |
// |  1.  |  steady@learner.com     |         100000        |
// |  2.  |  example@email.com      |         10000         |
