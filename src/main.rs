use rusqlite::{params, Connection, Result};
use std::error::Error;
use std::io::{self, Write};


mod parts_menu;

fn init_db() -> Result<Connection> {
    let conn = Connection::open("skate.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS drills (
            id INTEGER PRIMARY KEY,
            description TEXT NOT NULL,
            completed BOOLEAN NOT NULL DEFAULT 0
        )",
        [],
    )?;

    Ok(conn)
}

fn add_drill(conn: &Connection, description: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO drills(description, completed) VALUES (?1, ?2)",
        params![description, false],
    )?;
    Ok(())
}

fn show_drill(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("SELECT id, description, completed FROM drills ORDER BY completed")?;
    let drill_iter = stmt.query_map([], |row| {
        let completed: bool = row.get(2)?;
        Ok((row.get::<_, i32>(0)?, row.get::<_, String>(1)?, completed))
    })?;

    println!("Active drills:");
    for drill in drill_iter {
        let (id, description, completed) = drill?;
        if completed {
            println!("{}: {} [--Ok--]", id, description);
        } else {
            println!("{}: {} [✗]", id, description);
        }
    }

    Ok(())
}

fn complete_drill(conn: &Connection, drill_id: i32) -> Result<()> {
    conn.execute("UPDATE drills SET completed = 1 WHERE id = ?1", params![drill_id])?;
    Ok(())
}

fn delete_drill(conn: &Connection, drill_id: i32) -> Result<()> {
    conn.execute("DELETE FROM  drills WHERE id = ?1", params![drill_id])?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let conn = init_db()?;

    loop {
        
        println!("-----------------------------------------");
        println!("Track skate progession.");
        println!("-----------------------------------------");
        println!("1-> Add drill.");
        println!("2-> Show drill.");
        println!("3-> Complete drill.");
        println!("4-> Wish list menu.");
        println!("5-> Exit");
        println!("-----------------------------------------");

        let mut choice = String::new();
        println!("Enter your choice: ");
        io::stdout().flush()?;

        io::stdin().read_line(&mut choice)?;
        let choice = choice.trim();

        match choice {

        "1" => {
            let mut description = String::new();
            println!("Enter a drill: ");
            io::stdout().flush()?;
            io::stdin().read_line(&mut description)?;
            add_drill(&conn, description.trim())?;
        }

        "2" => {
            show_drill(&conn)?;
        }
        
        "3" => {
            let mut id = String::new();
            println!("Enter drill ID to complete: ");
            io::stdout().flush()?;
            io::stdin().read_line(&mut id)?;
            let id: i32 = id.trim().parse()?;
            complete_drill(&conn, id)?;
        }
        
        "4" => { parts_menu::second_menu();
      }

        "5" => break,
        _ => println!("Invalid choice!"),
        }
    }
    Ok(())
}
