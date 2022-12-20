pub enum MoveCommand {
    Down,
    Left,
    Right,
    Rotate,
}

impl TryFrom<String> for MoveCommand {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.trim() {
            "a" => Ok(MoveCommand::Left),
            "d" => Ok(MoveCommand::Right),
            "s" => Ok(MoveCommand::Rotate),
            ""  => Ok(MoveCommand::Down),
            _   => Err(()),
        }
    }
}

