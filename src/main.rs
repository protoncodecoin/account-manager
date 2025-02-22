use account_manager::welcome_msg;
use account_manager::UserAccount;
use std::io::stdin;

fn main() {
    let mut program_status: bool = true;
    let mut registered_accounts: Vec<UserAccount> = Vec::new();

    // println!("{}", user.get_account_info());

    while program_status {
        // show the welcome message to the user
        welcome_msg();

        // show a list of actions to the user
        let mut choice: String = String::new();
        println!("\n\n🤓: Hey there, I'm Toby.");
        println!("Select from the actions below to perforn an action\n");
        println!("1. Acount Creation\n(0) to quit::");
        stdin()
            .read_line(&mut choice)
            .expect("Couldn't read user choice");

        let choice: u8 = match choice.trim().parse::<u8>() {
            Ok(val) => val,
            Err(_) => continue, // should reselect action
        };

        // perform action based on user selection
        match choice {
            0 => {
                program_status = false;
            }
            1 => {
                println!("\n========= ACCOUNT CREATION =========\n");
                println!(
                    "🤓: Welcome to the account creation. I will guide you to create an account."
                );
                println!("🤓: What is your full name? ");
                let mut name: String = String::new();

                stdin()
                    .read_line(&mut name)
                    .expect("🤓: Sorry, Account Creation Failed");

                let validated_name: String = match name.trim().parse::<String>() {
                    Ok(val) => val,
                    Err(err) => {
                        println!("{:?}", err);
                        break;
                    }
                };

                let mut password: String = String::new();
                let mut validated_password: String = String::new();

                loop {
                    println!("🤓: Enter your password. Must be more than 8 characters long and should be alphanumeric💪");

                    // Remove characters from the password variable
                    password.clear();

                    stdin()
                        .read_line(&mut password)
                        .expect("🤓: Sorry, Account Creation Failed");

                    match password.trim().parse::<String>() {
                        Ok(new_password) => {
                            if new_password.len() < 8 {
                                println!("🤓: Your password is weak");
                                continue;
                            } else if new_password.is_empty() {
                                println!("🤓: Password field cannot be empty");
                                continue;
                            } else {
                                // return new password
                                println!("Your password is strong💪\n");
                                validated_password.push_str(&new_password);
                                break;
                            }
                        }
                        Err(err) => {
                            println!("🤓: An unexpected error occurred. {:?}", err);
                            break;
                        }
                    };
                }

                // create user account
                let registered_user: UserAccount =
                    UserAccount::new(validated_name, validated_password);
                registered_accounts.push(registered_user);

                // DEBUG:
                println!("{:?}", registered_accounts);
            }
            _ => {
                println!("Action not registrated. Quitting Program!");
                break;
            }
        }
    }
}
