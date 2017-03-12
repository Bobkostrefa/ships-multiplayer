use std::io;

struct Input {
    input: String
}
impl Input
{
    pub fn get_input() -> String
    {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Reading line failed");
        let input_trimmed = input.trim();
        return input_trimmed.to_string();
    }

}