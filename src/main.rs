use rusqlite::{params, Connection, Result};
use std::error::Error;
use std::io::{self, Write};

fn init_parts_db() -> Result<Connection> {
    let connn = Connection::open("parts.db")?;

    connn.execute(
        "CREATE TABLE IF NOT EXISTS parts (
            id INTEGER PRIMARY KEY,
            skatepart TEXT NOT NULL
        )",
        [],
    )?;
    
    Ok(connn)
}

fn add_skate_part(connn: &Connection, skatepart: &str) -> Result<()> {
    connn.execute(
        "INSERT INTO parts(skatepart) VALUES(?1)",
        params![skatepart],
    )?;
    Ok(())
}

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

fn calculate_price() {
    
    println!("Enter the amount of money you have: ");
    let mut money = String::new();
    io::stdin().read_line(&mut money).expect("Failed to read line");
    let money: f64 = money.trim().parse().expect("Please enter a valid number");

    println!("Enter the price of the item: ");
    let mut item = String::new();
    io::stdin().read_line(&mut item).expect("Failed to read line");
    let item: f64 = item.trim().parse().expect("Please enter a valid number");

    println!("Enter the day of the month: ");
    let mut day = String::new();
    io::stdin().read_line(&mut day).expect("Failed to read line");
    let day: f64 = day.trim().parse().expect("Please enter a valid number");


    let current_money: f64 = money - item;

    let days_left: f64 = 30.0 - day;

    let money_days: f64 = current_money / days_left;

    println!("If you buy this, you will have {} left in your bank", current_money);
    println!("There are {} days left in this month and you would have {:.1} to spend per day", days_left, money_days);


}
fn main() -> Result<(), Box<dyn Error>> {
    let conn = init_db()?;
    let connn = init_parts_db()?;

    loop {
        
        println!("Skate toolkit.");
        println!("-----------------------------------------");
        println!("1->Add drill.");
        println!("2->Show drill.");
        println!("3->Complete drill.");
        println!("-----------------------------------------");
        println!("Calculate budget to see if it a good buy.");
        println!("-----------------------------------------");
        println!("4-Calculate");
        println!("5-Add part to wishlist");
        println!("6-Show wishlist");
        println!("7-Remove from wishlist");
        println!("8->Exit");
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
        
//        "4" => { delete_drill(&conn)?;
//      }
        
        "5" => {
            let mut skatepart = String::new();
            println!("Enter a skate part that you want to buy: ");
            io::stdout().flush()?;
            io::stdin().read_line(&mut skatepart)?;
            add_skate_part(&connn, skatepart.trim())?;
        }

        "8" => break,
        _ => println!("Invalid choice!"),
        }
    }
    Ok(())
}
