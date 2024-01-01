//New years day project,this is a new year resolution tracker
use std::collections::HashMap;

fn main() {
    let mut resolutions = HashMap::new();

    loop {
        println!("New Year's Resolution Tracker");
        println!("1. Add resolution");
        println!("2. Remove resolution");
        println!("3. Mark resolution as complete");
        println!("4. List resolutions");
        println!("5. Exit");

        let choice = println!("Enter your choice: ");
        let choice = choice.trim().parse::<u32>().expect("Please enter a number");
        match choice {
            1 => {
                let resolution_text = println!("Enter your resolution: ");
                resolutions.insert(resolution_text, false);
                println!("Resolution added!");
            }
            2 => {
                let resolution_text = println!("Enter the resolution to remove: ");
                resolutions.remove(&resolution_text);
                println!("Resolution removed!");
            }
            3 => {
                let resolution_text = println!("Enter the resolution to mark as complete: ");
                resolutions.insert(resolution_text, true);
                println!("Resolution marked as complete!");
            }
            4 => {
                println!("Your resolutions:");
                for (resolution, completed) in &resolutions {
                    println!("- {} ({})", resolution, if *completed { "completed" } else { "pending" });
                }
            }
            5 => {
                break;
            }
            _ => {
                println!("Invalid choice. Please try again.");
            }
        }
    }
}
