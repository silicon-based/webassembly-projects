import * as wasm from "snake-game";

const CANVAS = <HTMLCanvasElement>document.getElementById("arena");
const HIGHESTSCORESPAN = document.getElementById("score-number-highest");
const CURRENTSCORESPAN = document.getElementById("score-number-current");
const CONTEXT = CANVAS.getContext("2d");
const GAME = wasm.Game.new(24);
const CELLSIZE = 30.0;

var drawSnakeOverload: NodeJS.Timeout[];

document.addEventListener("keydown", e => {
  GAME.handle_keystroke(e);
});


function redrawSnake() {
  let overloads = [];
  // Create a draw task for each pixel
  // * setTimeOut and setInterval does not block the thread *
  for (let l = 0; l <= CELLSIZE; l++) {
    let overload = drawWrapper(l);
    overloads.push(overload);
  }
  return overloads;
}


function drawWrapper(l: number) {
  let head = GAME.head_position();
  let tail = GAME.tail_position();
  let headDir = GAME.direction();
  let tailDir = GAME.tail_direction();
  let x = setTimeout(function() {
    wasm.draw(CONTEXT, head, headDir, tail, tailDir, CELLSIZE, l)
  }, l * 4);// wait for l * 4 ms so they can draw sequentially
  return x;
}


function run() {
  // setInterval: execute every 123ms
  setInterval(
    () => {
      // If game cannot continue, call initialize functions end continue to next loop
      if (GAME.update_and_check_continue() == false) {
        drawSnakeOverload.forEach((x) => { clearTimeout(x) }); // terminate all pending drawings
        GAME.initialize();
        CONTEXT.clearRect(0, 0, CANVAS.height, CANVAS.width);
        wasm.draw_init(CONTEXT, GAME, CELLSIZE);
        CURRENTSCORESPAN.innerHTML = "0";
        return;
      }
      GAME.draw_apple(CONTEXT, CELLSIZE);
      drawSnakeOverload = redrawSnake();
      CURRENTSCORESPAN.innerHTML = GAME.score().toString();
      if (+HIGHESTSCORESPAN.innerHTML < GAME.score()) {
        HIGHESTSCORESPAN.innerHTML = GAME.score().toString();
      }
    }, 125
  )
}


wasm.start();
wasm.draw_init(CONTEXT, GAME, CELLSIZE);
run();

