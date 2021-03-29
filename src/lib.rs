pub struct Game {
    pub board:Vec<Vec<char>>
}


pub struct Point {
    pub column:usize,
    pub row:usize,
}

impl Game {
    pub fn display_to_board(&self) {
        for column in self.board.iter() {
            print!("|");
            for row in column.iter() {
                print!("{} |",row);
            }
            println!();
        }
    }

    pub fn player_put_piece(&mut self,command:Point,piece:char) -> Result<(),&str> {
        let range:Vec<usize> = (1..4).collect();
        if range.contains(&command.column) && range.contains(&command.row){
            if self.board[command.column-1][command.row-1] != '■' {
                return Err("その場所には置けません");
            }
            self.board[command.column-1][command.row-1] = piece ;
        }else{
            return  Err("その場所には置けません");
        }
        Ok(())
    }

    pub fn becama_bingo(&self) -> Option<String> {
        for column in self.board.iter() {
            if column[0] == column[1] && column[1] == column[2] {
                if column[0] != '■' {
                    return Some(format!("Win! {}",column[0]));
                }
            }
        }

        for ci in 0..3 {
            if self.board[ci][0] == self.board[ci][1] 
            && self.board[ci][1] == self.board[ci][2] {
                if self.board[ci][0] != '■' {
                    return Some(format!("Win! {}",self.board[ci][0]));
                }
            }
        }
        
        if self.board[0][0] == self.board[1][1]
        && self.board[1][1] == self.board[2][2] {
            if self.board[0][0] != '■' {
                return Some(format!("Win! {}",self.board[0][0]))
            }
        }

        if self.board[0][2] == self.board[1][1]
        && self.board[1][1] == self.board[2][0] {
            if self.board[0][2] != '■' {
                return Some(format!("Win! {}",self.board[0][2]))
            }
        }

        None
    }
}


