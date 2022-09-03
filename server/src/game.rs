use tokio::sync::mpsc::UnboundedSender;
use warp::ws::Message;

#[derive(Default)]
pub struct Game {
    black: Option<Player>,
    white: Option<Player>,
    checkers: crate::checkers::Checkers,
}
impl Game {
    pub fn is_empty(&self) -> bool {
        self.black.is_none() && self.white.is_none()
    }
    pub fn add_player(&mut self, id: String, tx: UnboundedSender<Message>) -> Result<(), ()> {
        match (&self.black, &self.white) {
            (None, _) => {
                tx.send(Message::text("black")).map_err(|_| ())?;
                self.black = Some(Player { id, tx });
            }
            (Some(_), None) => {
                tx.send(Message::text("white")).map_err(|_| ())?;
                self.white = Some(Player { id, tx });
            }
            _ => {
                tx.send(Message::text("full")).map_err(|_| ())?;
                return Err(());
            }
        }
        self.broadcast();
        Ok(())
    }
    pub fn remove_player(&mut self, id: &str) {
        match (&self.black, &self.white) {
            (Some(black), _) if black.id == id => self.black = None,
            (_, Some(white)) if white.id == id => self.white = None,
            _ => (),
        }
    }
    pub fn try_move_piece(&mut self, id: &str, msg: Message) {
        match (&self.black, &self.white, msg.to_str()) {
            // Player connected and message parsed well
            (Some(black), Some(white), Ok(msg))
            // Correct turn
                if ((black.id == id && self.checkers.is_black_turn())
                    || (white.id == id && !self.checkers.is_black_turn()))
            // Move successful
                    && self.checkers.try_move_piece(msg).is_ok() =>
            {
                self.broadcast()
            }
            _ => return,
        }
    }
    fn broadcast(&self) {
        let msg = self.checkers.to_string();
        if let Some(black) = &self.black {
            black.tx.send(Message::text(msg.clone())).ok();
        }
        if let Some(white) = &self.white {
            white.tx.send(Message::text(msg)).ok();
        }
    }
}

struct Player {
    id: String,
    tx: UnboundedSender<Message>,
}
