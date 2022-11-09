import * as wasm from "snake-game";

const canvas = <HTMLCanvasElement>document.getElementById("arena");
const highestScoreSpan = document.getElementById("highest-score");
const nowScoreSpan = document.getElementById("now-score");
const context = canvas.getContext("2d");
const game = wasm.Game.new(24);
const cellSize = 30.0;

var drawSnakeOverload: NodeJS.Timeout[];

document.addEventListener("keydown", e => {
  game.handle_keystroke(e);
});


function redrawSnake(head: wasm.Cell, headDir: wasm.Direction, tail: wasm.Cell, tailDir: number, cellSize: number) {
  let overloads = [];
  // Create a draw task for each pixel
  // * setTimeOut and setInterval does not block the thread *
  for (let l = 0; l <= cellSize; l++) {
    let overload = drawWrapper(head, headDir, tail, tailDir, cellSize, l);
    overloads.push(overload);
  }
  return overloads;
}


function drawWrapper(head: wasm.Cell, headDir: wasm.Direction, tail: wasm.Cell, tailDir: number, cellSize: number, l: number) {
  let x = setTimeout(function() {
    wasm.draw(context, head, headDir, tail, tailDir, cellSize, l)
  }, l * 4);// wait for l * 4 ms so they can draw sequentially
  return x;
}


function run() {
  // setInterval: execute every 123ms
  setInterval(
    () => {
      // If game cannot continue, call initialize functions end continue to next loop
      if (game.update_and_check_continue() == false) {
        drawSnakeOverload.forEach((x) => { clearTimeout(x) }); // terminate all pending drawings
        game.initialize();
        context.clearRect(0, 0, canvas.height, canvas.width);
        wasm.draw_init(game, cellSize, context);
        nowScoreSpan.innerHTML = "0";
        return;
      }
      let head = game.head_position();
      let tail = game.tail_position();
      game.draw_apple(context, cellSize);
      drawSnakeOverload = redrawSnake(head, game.direction(),
        tail, game.tail_direction(), cellSize);
      nowScoreSpan.innerHTML = game.score().toString();
      if (+highestScoreSpan.innerHTML < game.score()) {
        highestScoreSpan.innerHTML = game.score().toString();
      }
    }, 123
  )
}


wasm.start();
wasm.draw_init(game, cellSize, context);
run();

