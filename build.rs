fn main() {
    println!("Inside build.rs. Should I use the function in db/sqlite.rs here?");
}

// https://github.com/steadylearner/Rust-Full-Stack/tree/master/auth/javascript/express/db/sql

// id uuid PRIMARY KEY DEFAULT uuid_generate_v4 (),
// password VARCHAR NOT NULL CHECK (char_length(password) >= 5)
// email VARCHAR(255) UNIQUE NOT NULL
//   CONSTRAINT valid_email CHECK (email ~* '^[A-Za-z0-9._%-]+@[A-Za-z0-9.-]+[.][A-Za-z]+$'),
//   password VARCHAR NOT NULL CHECK (char_length(password) >= 5),
