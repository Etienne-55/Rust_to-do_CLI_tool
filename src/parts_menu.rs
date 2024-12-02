use rusqlite::{params, Connection, Result};
use std::error::Error;
use std::io::{self, Write};

pub fn init_parts_db() -> Result<Connection> {
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

pub fn add_skate_part(connn: &Connection, skatepart: &str) -> Result<()> {
    connn.execute(
        "INSERT INTO parts(skatepart) VALUES(?1)",
        params![skatepart],
    )?;
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
        println!("4-Remove from wishlist");
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

        "5" => break,
        _ => println!("Invalid choice!"),
        }
    }
    Ok(())
}
