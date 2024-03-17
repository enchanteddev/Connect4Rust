<script>
// @ts-nocheck

    import Board from "./Board.svelte";

    export let get_move;

    let board = [
      [0, 0, 0, 0, 0, 0, 0],
      [0, 0, 0, 0, 0, 0, 0],
      [0, 0, 0, 0, 0, 0, 0],
      [0, 0, 0, 0, 0, 0, 0],
      [0, 0, 0, 0, 0, 0, 0],
      [0, 0, 0, 0, 0, 0, 0]
    ]

    function dropPiece(row, player) {
        if (board[0][row] !== 0) {
            return false;
        }
        for (let i = 5; i >= 0; i--) {
            if (board[i][row] === 0) {
                board[i][row] = player;
                return true;
            }
        }
        return false;
    }

    function flatten2DArray(arr) {
      let result = [];

      function flatten(arr) {
          arr.forEach(item => {
              if (Array.isArray(item)) {
                  flatten(item);
              } else {
                  result.push(item);
              }
          });
      }

      flatten(arr);
      return result;
    }
    function checkWinner(board) {
      // Check horizontal
      for (let row = 0; row < 6; row++) {
          for (let col = 0; col < 4; col++) {
              if (board[row][col] !== 0 &&
                  board[row][col] === board[row][col + 1] &&
                  board[row][col] === board[row][col + 2] &&
                  board[row][col] === board[row][col + 3]) {
                  return board[row][col]; // Return the player number who won
              }
          }
      }

      // Check vertical
      for (let row = 0; row < 3; row++) {
          for (let col = 0; col < 7; col++) {
              if (board[row][col] !== 0 &&
                  board[row][col] === board[row + 1][col] &&
                  board[row][col] === board[row + 2][col] &&
                  board[row][col] === board[row + 3][col]) {
                  return board[row][col]; // Return the player number who won
              }
          }
      }

      // Check diagonals (top-left to bottom-right)
      for (let row = 0; row < 3; row++) {
          for (let col = 0; col < 4; col++) {
              if (board[row][col] !== 0 &&
                  board[row][col] === board[row + 1][col + 1] &&
                  board[row][col] === board[row + 2][col + 2] &&
                  board[row][col] === board[row + 3][col + 3]) {
                  return board[row][col]; // Return the player number who won
              }
          }
      }

      // Check diagonals (bottom-left to top-right)
      for (let row = 3; row < 6; row++) {
          for (let col = 0; col < 4; col++) {
              if (board[row][col] !== 0 &&
                  board[row][col] === board[row - 1][col + 1] &&
                  board[row][col] === board[row - 2][col + 2] &&
                  board[row][col] === board[row - 3][col + 3]) {
                  return board[row][col]; // Return the player number who won
              }
          }
      }

      return 0; // If no winner found
  }

    let player = 1;
    let msg = "Your Turn"
    let blocked = false;
    let won = false;
</script>
<h1>{msg}</h1>
<Board board={board} move={(row) => {
  if (blocked || won) {
    console.log("wad")
    return
  }
  console.log(blocked)
  dropPiece(row, player)
  let w = checkWinner(board);
  msg = "AI's Turn"
  if (w != 0){
    msg = `${w == 1 ? "You" : "AI"} won!`
    won = true;
  }
  let enemy = player == 1 ? 2 : 1
  function startAI() {
    let move = get_move(flatten2DArray(board), 6, enemy)
    dropPiece(move, enemy)
    let w = checkWinner(board);
    msg = "Your Turn"
    if (w != 0){
      msg = `${w == 1 ? "You" : "AI"} won!`
      won = true;
    }
    blocked = false;
  }
  blocked = true;
  setTimeout(startAI, 5)
}}/>