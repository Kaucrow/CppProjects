pub struct HelpText {
    pub login: LoginHelpText,
    pub admin: AdminHelpText,
    pub client: ClientHelpText
}

impl HelpText {
    pub const fn default() -> Self {
        HelpText {
            login: LoginHelpText {
                main: "Press `Alt` to switch input.",
                login_failed: "Login failed.",
                login_failed_lock: "Login failed. - Try again in: ",
            },
            admin: AdminHelpText {
                main_left: "Choose an action to perform. `Alt`: Switch windows. `Esc`: Go back.",
                main_right: "Choose a client to edit its data. `Alt`: Switch windows. `Esc`: Go back.",
                filter_left: "Choose a filter to edit. `a`: Apply the selected filters. `Esc`: Go back.",
                filter_right: "Input the value. `Enter`: Save changes. `Esc`: Quit editing and don't save changes.",
                add_client_left: "Choose a client data field to add. `r`: Register the client. `Esc`: Go back.",
                add_client_right: "Input the value. `Enter`: Save changes. `Esc`: Quit editing and don't save changes.",
                missing_cltfield: "Missing data field for ",
            },
            client: ClientHelpText {
                main: "Choose an action to perform. `Esc`: Go back.",
                deposit: "Input the amount to deposit. `Enter`: Perform the transaction.",
                withdraw: "Input the amount to withdraw. `Enter`: Perform the transaction.",
                transfer: "Input the amount to transfer and the beneficiary. `Enter`: Perform the transaction.",
                change_psswd: "Input your current and new password. `Enter`: Update the password.",
                unknown_beneficiary: "The beneficiary could not be found.",
                incorrect_password: "Incorrect \"current password\".",
                transfer_to_self: "You can't transfer money to yourself.",
                not_enough_money: "You don't have enough money."
            }
        }
    }
}

pub struct LoginHelpText {
    pub main: &'static str,
    pub login_failed: &'static str,
    pub login_failed_lock: &'static str,
}

pub struct ClientHelpText {
    pub main: &'static str,
    pub deposit: &'static str,
    pub withdraw: &'static str,
    pub transfer: &'static str,
    pub change_psswd: &'static str,
    pub unknown_beneficiary: &'static str,
    pub incorrect_password: &'static str,
    pub transfer_to_self: &'static str,
    pub not_enough_money: &'static str,

}

pub struct AdminHelpText {
    pub main_left: &'static str,
    pub main_right: &'static str,
    pub filter_left: &'static str,
    pub filter_right: &'static str,
    pub add_client_left: &'static str,
    pub add_client_right: &'static str,
    pub missing_cltfield: &'static str,
}