use std::io;
pub mod Message
{
   pub static ENTER_IP: &'static str = "Please enter desired IP address. For example: 192.168.0.1:4444 - [ip]:[port]";
   pub static ENTER_NICKNAME: &'static str = "Please enter your nickname";
   pub static ENTER_IP_AGAIN: &'static str = "Couldnt connect to the desired server. Reason:";
   pub static ENTER_CONNECTION: &'static str = "Trying to connect. It may take a while. Do not panic";
   pub static ENTER_RETRY: &'static str = "\r\nDo you want to try reconnecting?\r\nYes - Retry connection\r\nNo - Change ip address";
   pub static ENEMY_HIT: &'static str = "Your enemy hit your ship";
   pub static ENEMY_MISS: &'static str = "Enemy missed";
   pub static ENEMY_DISCONNECTED: &'static str = "Your enemy disconnected";
   pub static PLAYER_HIT: &'static str = "Your shot hit an enemy ship!";
   pub static PLAYER_MISS: &'static str = "Your shot missed. lamus";
   pub static PLAYER_WON: &'static str = "Congratulations! You won!";
   pub static PLAYER_LOST: &'static str = "You lost. Maybe try next time!";
   pub static AWAITING_RESPONSE: &'static str = "Awaiting response from your enemy";
}

pub fn send_message_with_input(message: &'static str) -> String
    {
        println!("{}", message);
        let input = get_input();
        return input;
    }
pub fn send_message_with_input_string(message: String) -> String
    {
        println!("{}", message);
        let input = get_input();
        return input;
    }
pub fn send_message_str(message: &'static str)
    {
        println!("{}", message);
    }
pub fn send_message_string(message: String)
    {
        println!("{}", message);
    }
pub fn get_input() -> String
    {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Reading line failed");
        let input_trimmed = input.trim();
        return input_trimmed.to_string();
    }
