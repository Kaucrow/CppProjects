pub struct HelpText {
    pub admin: AdminHelpText,
    pub client: ClientHelpText
}

impl HelpText {
    pub const fn default() -> Self {
        HelpText {
            admin: AdminHelpText {
                main_left: "Choose an action to perform. `Alt`: Switch windows. `Esc`: Go back.",
                main_right: "Choose a client to edit its data. `Alt`: Switch windows. `Esc`: Go backk.",
                filter_left: "Choose a filter to edit. `a`: Apply the selected filters. `Esc`: Go back.",
                filter_right: "Input the value. `Enter`: Save changes. `Esc`: Quit editing and don't save changes."
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
}