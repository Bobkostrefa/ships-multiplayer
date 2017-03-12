use std::net::{TcpStream, Shutdown};
use messages;
use messages::Message;
enum ClientState
{
    Nickname,
    IpAddress,


    Done
}
pub struct Client {
    Nickname: String,
    IpAddress: String,
    Stream: TcpStream,
    State: ClientState,
    
}
impl Client
{
    pub fn new() -> Client
    {
        let connection_test = TcpStream::connect("8.8.8.8:53").expect("No internet connection");
        Client
        {
            Nickname: String::new(),
            IpAddress: String::new(),
            Stream: connection_test,
            State: ClientState::Nickname,
        }
    }
    pub fn get_input(&mut self)
    {
        'state: loop
        {
        match self.State
            {
                ClientState::Nickname =>
                {
                    let input = messages::send_message_with_input(Message::ENTER_NICKNAME);
                    self.Nickname = input;
                    self.State = ClientState::IpAddress;
                }
                ClientState::IpAddress =>
                {
                    let input = messages::send_message_with_input(Message::ENTER_IP);
                    self.IpAddress = input;
                    self.State = ClientState::Done;
                }
                ClientState::Done =>
                {
                    messages::send_message_str(Message::ENTER_CONNECTION);
                    let connection = self.create_connection();
                    match connection
                    {
                        Err(e) =>
                        {
                            let input = messages::send_message_with_input_string(e);
                            if input == "Yes" || input == "yes" || input == "y" || input == "Y"
                            {
                                self.State = ClientState::Done;
                            }
                            else 
                            {
                                self.State = ClientState::IpAddress;
                            }

                        }
                        Ok(stream) =>
                        {
                            self.Stream = stream;
                        }
                    }
                }


            }
        }
    }
    pub fn create_connection(&mut self) -> Result<TcpStream, String>
    {
        self.Stream.shutdown(Shutdown::Both);
    let mut stream = TcpStream::connect(&self.IpAddress[..]);
    match stream
    {
        Err(e) => 
        {
            let message = format!("{} {}{}", Message::ENTER_IP_AGAIN, e, Message::ENTER_RETRY);
            Err(message)
        }
        Ok(stream) =>
        {
            Ok(stream)
        }
    }
    }
    
}