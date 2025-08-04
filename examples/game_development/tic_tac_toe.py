# @depyler: optimization_level = "aggressive"
# @depyler: bounds_checking = "explicit"
from typing import List, Optional, Tuple

class TicTacToe:
    """Tic-tac-toe game implementation"""
    
    def __init__(self) -> None:
        self.board: List[List[str]] = [
            [" ", " ", " "],
            [" ", " ", " "],
            [" ", " ", " "]
        ]
        self.current_player = "X"
    
    def make_move(self, row: int, col: int) -> bool:
        """Make a move at specified position"""
        if not self.is_valid_move(row, col):
            return False
        
        self.board[row][col] = self.current_player
        self.current_player = "O" if self.current_player == "X" else "X"
        return True
    
    def is_valid_move(self, row: int, col: int) -> bool:
        """Check if move is valid"""
        if row < 0 or row >= 3 or col < 0 or col >= 3:
            return False
        return self.board[row][col] == " "
    
    def check_winner(self) -> Optional[str]:
        """Check if there's a winner"""
        # Check rows
        for row in self.board:
            if row[0] == row[1] == row[2] != " ":
                return row[0]
        
        # Check columns
        for col in range(3):
            if self.board[0][col] == self.board[1][col] == self.board[2][col] != " ":
                return self.board[0][col]
        
        # Check diagonals
        if self.board[0][0] == self.board[1][1] == self.board[2][2] != " ":
            return self.board[0][0]
        
        if self.board[0][2] == self.board[1][1] == self.board[2][0] != " ":
            return self.board[0][2]
        
        return None
    
    def is_board_full(self) -> bool:
        """Check if board is full"""
        for row in self.board:
            for cell in row:
                if cell == " ":
                    return False
        return True
    
    def is_game_over(self) -> bool:
        """Check if game is over"""
        return self.check_winner() is not None or self.is_board_full()
    
    def get_empty_positions(self) -> List[Tuple[int, int]]:
        """Get all empty positions on the board"""
        positions: List[Tuple[int, int]] = []
        for row in range(3):
            for col in range(3):
                if self.board[row][col] == " ":
                    positions.append((row, col))
        return positions
    
    def board_to_string(self) -> str:
        """Convert board to string representation"""
        lines: List[str] = []
        for i, row in enumerate(self.board):
            line = f" {row[0]} | {row[1]} | {row[2]} "
            lines.append(line)
            if i < 2:
                lines.append("-----------")
        return "\n".join(lines)

class AIPlayer:
    """Simple AI player using minimax algorithm"""
    
    def __init__(self, symbol: str) -> None:
        self.symbol = symbol
        self.opponent = "O" if symbol == "X" else "X"
    
    def get_best_move(self, game: TicTacToe) -> Tuple[int, int]:
        """Get best move using minimax"""
        best_score = -1000
        best_move = (0, 0)
        
        for row, col in game.get_empty_positions():
            # Make temporary move
            game.board[row][col] = self.symbol
            score = self.minimax(game, 0, False)
            # Undo move
            game.board[row][col] = " "
            
            if score > best_score:
                best_score = score
                best_move = (row, col)
        
        return best_move
    
    def minimax(self, game: TicTacToe, depth: int, is_maximizing: bool) -> int:
        """Minimax algorithm implementation"""
        winner = game.check_winner()
        
        if winner == self.symbol:
            return 10 - depth
        elif winner == self.opponent:
            return depth - 10
        elif game.is_board_full():
            return 0
        
        if is_maximizing:
            best_score = -1000
            for row, col in game.get_empty_positions():
                game.board[row][col] = self.symbol
                score = self.minimax(game, depth + 1, False)
                game.board[row][col] = " "
                best_score = max(best_score, score)
            return best_score
        else:
            best_score = 1000
            for row, col in game.get_empty_positions():
                game.board[row][col] = self.opponent
                score = self.minimax(game, depth + 1, True)
                game.board[row][col] = " "
                best_score = min(best_score, score)
            return best_score

def play_computer_game() -> str:
    """Play a complete game against computer"""
    game = TicTacToe()
    ai = AIPlayer("O")
    moves_log: List[str] = []
    
    while not game.is_game_over():
        if game.current_player == "X":
            # Human player - make a simple move (first available)
            empty_positions = game.get_empty_positions()
            if empty_positions:
                row, col = empty_positions[0]
                game.make_move(row, col)
                moves_log.append(f"Human plays at ({row}, {col})")
        else:
            # AI player
            row, col = ai.get_best_move(game)
            game.make_move(row, col)
            moves_log.append(f"AI plays at ({row}, {col})")
    
    winner = game.check_winner()
    if winner:
        moves_log.append(f"Winner: {winner}")
    else:
        moves_log.append("Game ended in a tie")
    
    return "\n".join(moves_log)

def count_winning_positions(board_state: List[List[str]], player: str) -> int:
    """Count how many ways a player can win from current state"""
    winning_positions = [
        # Rows
        [(0, 0), (0, 1), (0, 2)],
        [(1, 0), (1, 1), (1, 2)],
        [(2, 0), (2, 1), (2, 2)],
        # Columns
        [(0, 0), (1, 0), (2, 0)],
        [(0, 1), (1, 1), (2, 1)],
        [(0, 2), (1, 2), (2, 2)],
        # Diagonals
        [(0, 0), (1, 1), (2, 2)],
        [(0, 2), (1, 1), (2, 0)]
    ]
    
    count = 0
    for positions in winning_positions:
        can_win = True
        for row, col in positions:
            cell = board_state[row][col]
            if cell != " " and cell != player:
                can_win = False
                break
        if can_win:
            count += 1
    
    return count