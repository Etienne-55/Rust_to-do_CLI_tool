use rusqlite::{params, Connection, Result};
use std::error::Error;
use std::io::{self, Write};

pub fn init_parts_db() -> Result<Connection> {
    let connn = Connection::open("parts-skate.db")?;

    connn.execute(
        "CREATE TABLE IF NOT EXISTS parts (
            id INTEGER PRIMARY KEY,
            skatepart TEXT NOT NULL,
            bought BOOLEAN NOT NULL DEFAULT 0
        )",
        [],
    )?;
    
    Ok(connn)
}

pub fn add_skate_part(connn: &Connection, skatepart: &str) -> Result<()> {
    connn.execute(
        "INSERT INTO parts(skatepart, bought) VALUES(?1, ?2)",
        params![skatepart, false],
    )?;
    Ok(())
}

fn show_parts(connn: &Connection) -> Result<()> {
    let mut tmt = connn.prepare("SELECT id, skatepart, bought FROM parts ORDER BY bought")?;
    let part_iter = tmt.query_map([], |row| {
        let bought: bool = row.get(2)?;
        Ok((row.get::<_, i32>(0)?, row.get::<_, String>(1)?, bought))
    })?;

    println!("Parts to buy:");
    for part in part_iter {
        let (id, skatepart, bought) = part?;
        if bought {
            println!("{}: {} [--bought--]", id, skatepart);
        } else {
            println!("{}: {} [✗]", id, skatepart);
        }
    }

    Ok(())
}

fn bought_parts(connn: &Connection, part_id: i32) -> Result<()> {
    connn.execute("UPDATE parts SET bought = 1 WHERE id = ?1", params![part_id])?;
    Ok(())
}


pub fn calculate_price() {
    
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
pub fn second_menu() -> Result<(), Box<dyn Error>> {
    let connn = init_parts_db()?;

    loop {
        
        println!("-----------------------------------------");
        println!("Calculate budget to see if it a good buy.");
        println!("-----------------------------------------");
        println!("1-Calculate budget");
        println!("2-Add part to wishlist");
        println!("3-Show wishlist");
        println!("4-Mark as bought");
        println!("5->Exit");
        println!("-----------------------------------------");

        let mut choice = String::new();
        println!("Enter your choice: ");
        io::stdout().flush()?;

        io::stdin().read_line(&mut choice)?;
        let choice = choice.trim();

        match choice {

        "1" => {  
            calculate_price();
        }

        "2" => {
            let mut skatepart = String::new();
            println!("Enter a skate part that you want to buy: ");
            io::stdout().flush()?;
            io::stdin().read_line(&mut skatepart)?;
            add_skate_part(&connn, skatepart.trim())?;
        }

        "3" => {
            show_parts(&connn)?;
        }

        "4" => {
            let mut id = String::new();
            println!("Enter item ID to mark as bought: ");
            io::stdout().flush()?;
            io::stdin().read_line(&mut id)?;
            let id: i32 = id.trim().parse()?;
            bought_parts(&connn, id)?;
        }

        "5" => break,
        _ => println!("Invalid choice!"),
        }
    }
    Ok(())
}
