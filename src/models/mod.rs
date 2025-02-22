/// Model User Account Type
#[derive(Debug)]
pub enum AccountType {
    AdminAccount,
    GuestAccount,
    RegularAccount,
    DeviceAccount,
    SoftwareAccount,
    ThirdPartyAccount,
}

impl AccountType {
    /// Get account type information
    pub fn get_account_type_info(&self) -> () {
        match self {
            Self::AdminAccount => println!("Admin permissions"),
            Self::GuestAccount => println!("Guess User permissions"),
            Self::RegularAccount => println!("Regular User permissions"),
            Self::DeviceAccount => println!("Similar to Regular User permissions"),
            Self::SoftwareAccount => println!("Required permission for a software to run"),
            Self::ThirdPartyAccount => {
                println!("Permissions needed for a third party to use the network")
            }
        }
    }
    /// Get the string representation of the acoount type
    fn get_str_representation(&self) -> &str {
        match self {
            Self::AdminAccount => "Admin Account",
            Self::GuestAccount => "Guest Account",
            Self::RegularAccount => "Regular Account",
            Self::DeviceAccount => "Device Account",
            Self::SoftwareAccount => "Softare Account",
            Self::ThirdPartyAccount => "Third Party Account",
        }
    }
}

/// Model User Account
#[derive(Debug)]
pub struct UserAccount {
    name: String,
    password: String,
    account_type: AccountType,
}

impl UserAccount {
    /// create new user account
    pub fn new(name: String, password: String) -> Self {
        Self {
            name,
            password,
            account_type: AccountType::RegularAccount,
        }
    }

    /// Get User account information
    pub fn get_account_info(&self) -> String {
        let name: &String = &self.name;
        let account_type = &self.account_type;
        format!(
            "User: {}, Account Type: {}",
            name,
            account_type.get_str_representation()
        )
    }
}
