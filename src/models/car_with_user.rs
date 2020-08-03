use rusqlite::{params, Connection, Result, NO_PARAMS};

#[derive(Debug)]
pub struct CarWithUser {
    pub email: String,
    pub user_id: i64,
    pub car_id: i64,
    pub price: f64,
    pub color: String,
}

impl CarWithUser {
    pub fn get(conn: &Connection, id: i64) -> Result<Vec<CarWithUser>> {
        let mut stmt = conn.prepare(
            "
            SELECT users.email, users.id, cars.id, price, color
            FROM cars
                INNER JOIN users ON users.id = cars.user_id
                WHERE cars.id = (?1);
        ",
        )?;

        let result = stmt.query_map(params![&id], |row| {
            Ok(CarWithUser {
                email: row.get(0)?,
                user_id: row.get(1)?,
                car_id: row.get(2)?,
                price: row.get(3)?,
                color: row.get(4)?,
            })
        })?;

        let mut car_with_user = Vec::new();
        for c in result {
            car_with_user.push(c?);
        }

        Ok(car_with_user)
    }
}

#[derive(Debug)]
pub struct CarWithUserList(pub Vec<CarWithUser>);

impl CarWithUserList {
    pub fn list(conn: &Connection) -> Result<Vec<CarWithUser>> {
        let mut stmt = conn.prepare(
            "
            SELECT users.email, users.id, cars.id, price, color
            FROM cars
                INNER JOIN users ON users.id = cars.user_id;
        ",
        )?;

        let result = stmt.query_map(NO_PARAMS, |row| {
            Ok(CarWithUser {
                email: row.get(0)?,
                user_id: row.get(1)?,
                car_id: row.get(2)?,
                price: row.get(3)?,
                color: row.get(4)?,
            })
        })?;

        let mut cars_with_users = Vec::new();
        for c in result {
            cars_with_users.push(c?);
        }

        Ok(cars_with_users)
    }
}
