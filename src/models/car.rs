use rusqlite::{params, Connection, Result, NO_PARAMS};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NewCar {
    pub price: f64,
    pub color: String,
    pub user_id: i64,
}

impl Default for NewCar {
    fn default() -> Self {
        NewCar {
            price: 10000 as f64,
            color: "black".into(),
            user_id: 1, // SQLIte id starts with 1.
        }
    }
}

// 1. Make a car with the user id as foreign key of it.
// 2. Minus cash to the price of the car.
impl NewCar {
    pub fn create(&self, conn: &mut Connection) -> Result<()> {
        let tx = conn.transaction()?;

        tx.execute(
            "insert into cars (price, color, user_id) values (?1, ?2, ?3)",
            &[
                &self.price.to_string(),
                &self.color,
                &self.user_id.to_string(),
            ],
        )?;
        tx.execute(
            "UPDATE users SET cash = cash - (?1) WHERE id = (?2);",
            &[&self.price.to_string(), &self.user_id.to_string()],
        )?;

        tx.commit()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewCarRequest {
    pub price: f64,
    pub color: String,
}

#[derive(Debug)]
pub struct Car {
    pub id: i64,
    pub price: f64,
    pub color: String,
    pub user_id: i64,
    // Include it later if you want.
    // pub created_at: NaiveDateTime,
    // pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CarPublic {
    pub price: f64,
    pub color: String,
}

impl Car {
    pub fn get(conn: &Connection, id: &i64) -> Result<Vec<Car>> {
        let mut stmt = conn.prepare("SELECT * FROM cars WHERE id = (?1);")?;

        let result = stmt.query_map(params![&id.to_owned()], |row| {
            Ok(Car {
                id: row.get(0)?,
                price: row.get(1)?,
                color: row.get(2)?,
                user_id: row.get(3)?,
            })
        })?;

        let mut car = Vec::new();
        for c in result {
            car.push(c?);
        }

        Ok(car)
    }

    pub fn refund(&self, conn: &mut Connection) -> Result<()> {
        let tx = conn.transaction()?;

        tx.execute(
            "UPDATE users SET cash = cash + (?1) WHERE id = (?2);",
            &[&self.price.to_string(), &self.user_id.to_string()],
        )?;
        tx.execute("DELETE FROM cars WHERE id = (?1)", &[&self.id])?;

        tx.commit()
    }

    pub fn delete(conn: &Connection, id: &i64) -> Result<()> {
        conn.execute("DELETE FROM cars WHERE id = (?1);", &[&id.to_owned()])?;
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CarRefundRequest {
    pub car_id: i64,
}

#[derive(Debug)]
pub struct CarList(pub Vec<Car>);

impl CarList {
    pub fn list(conn: &Connection) -> Result<Vec<Car>> {
        let mut stmt = conn.prepare("SELECT * FROM cars;")?;

        let result = stmt.query_map(NO_PARAMS, |row| {
            Ok(Car {
                id: row.get(0)?,
                price: row.get(1)?,
                color: row.get(2)?,
                user_id: row.get(3)?,
            })
        })?;

        let mut cars = Vec::new();
        for u in result {
            cars.push(u?);
        }
        // println!("{:#?}", cars);

        Ok(cars)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CarPublicList(pub Vec<CarPublic>);

impl CarPublicList {
    pub fn list(conn: &Connection, user_id: &i64) -> Result<Vec<CarPublic>> {
        let mut stmt = conn.prepare("SELECT price, color FROM cars where user_id = (?1)")?;

        let result = stmt.query_map(params![&user_id], |row| {
            Ok(CarPublic {
                price: row.get(0)?,
                color: row.get(1)?,
            })
        })?;

        let mut cars = Vec::new();
        for u in result {
            cars.push(u?);
        }
        // println!("{:#?}", cars);

        Ok(cars)
    }
}

