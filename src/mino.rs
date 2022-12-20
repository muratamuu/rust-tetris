use crate::move_command::*;
use crate::utils::*;

#[derive(Debug, Clone)]
pub enum MinoType {
    I, O, S, Z, J, L, T,
}

impl MinoType {
    pub fn pattern(&self) -> &'static [[(i8, i8); 4]] {
        match self {
            Self::I => { const PAT: [[(i8, i8); 4]; 2] = [
                [( 0,  0), ( 0, -1), ( 0,  1), ( 0,  2)],
                [( 0,  0), (-1,  0), ( 1,  0), ( 2,  0)],
            ]; &PAT },
            Self::O => { const PAT: [[(i8, i8); 4]; 1] = [
                [( 0,  0), ( 1,  0), ( 0, -1), ( 1, -1)],
            ]; &PAT },
            Self::S => { const PAT: [[(i8, i8); 4]; 2] = [
                [( 0,  0), (-1,  0), ( 0, -1), ( 1, -1)],
                [( 0,  0), ( 0, -1), ( 1,  0), ( 1,  1)],
            ]; &PAT },
            Self::Z => { const PAT: [[(i8, i8); 4]; 2] = [
                [( 0,  0), ( 1,  0), ( 0, -1), (-1, -1)],
                [( 0,  0), ( 0,  1), ( 1,  0), ( 1, -1)],
            ]; &PAT },
            Self::J => { const PAT: [[(i8, i8); 4]; 4] = [
                [( 0,  0), ( 0, -1), ( 0,  1), (-1,  1)],
                [( 0,  0), ( 1,  0), (-1,  0), (-1, -1)],
                [( 0,  0), ( 0,  1), ( 0, -1), ( 1, -1)],
                [( 0,  0), (-1,  0), ( 1,  0), ( 1,  1)],
            ]; &PAT },
            Self::L => { const PAT: [[(i8, i8); 4]; 4] = [
                [( 0,  0), ( 0, -1), ( 0,  1), ( 1,  1)],
                [( 0,  0), ( 1,  0), (-1,  0), (-1,  1)],
                [( 0,  0), ( 0,  1), ( 0, -1), (-1, -1)],
                [( 0,  0), (-1,  0), ( 1,  0), ( 1, -1)],
            ]; &PAT },
            Self::T => { const PAT: [[(i8, i8); 4]; 4] = [
                [( 0,  0), ( 0, -1), ( 1,  0), (-1,  0)],
                [( 0,  0), ( 1,  0), ( 0,  1), ( 0, -1)],
                [( 0,  0), ( 0,  1), (-1,  0), ( 1,  0)],
                [( 0,  0), (-1,  0), ( 0, -1), ( 0,  1)],
            ]; &PAT },
        }
    }
}

#[derive(Debug, Clone)]
pub struct Mino {
    pub type_: MinoType,
    pub pos: Pos,
    pub rotation: usize,
}

impl Mino {
    pub fn random() -> Self {
        use rand::prelude::*;
        let type_ = match rand::thread_rng().gen_range(0, 7) {
            0 => MinoType::I,
            1 => MinoType::O,
            2 => MinoType::S,
            3 => MinoType::Z,
            4 => MinoType::J,
            5 => MinoType::L,
            6 => MinoType::T,
            _ => MinoType::I,
        };
        Self { type_: type_, pos: (5, 1), rotation: 0 }
    }

    pub fn positions(&self) -> Vec<Pos> {
        let pat_idx = self.rotation % self.type_.pattern().len();
        self.type_.pattern()[pat_idx].iter()
            .map(|(dx, dy)| (self.pos.0 + dx, self.pos.1 + dy))
            .collect()
    }

    pub fn moved(&self, command: MoveCommand) -> Mino {
        let mut mino = self.clone();
        match command {
            MoveCommand::Left   => mino.pos.0 -= 1,
            MoveCommand::Right  => mino.pos.0 += 1,
            MoveCommand::Down   => mino.pos.1 += 1,
            MoveCommand::Rotate => mino.rotation += 1,
        }
        mino
    }
}

