import * as wasmModule from "sudoku-solver";

let gridData: Uint8Array = new Uint8Array(81);

function createGrid() {
  var input_grid = document.getElementById('input-grid');
  for (var j = 0; j < 81; j++) {
    const cell = document.createElement('div');
      if ((j % 27 > 17) && (j % 27 < 27)) {
      cell.classList.add("cell-border-bottom");
    }
    cell.classList.add('cell');
    const input = document.createElement('input');
    input.classList.add('cell-input')
    input.type = "number";
    input.min = "1";
    input.maxLength = 1;
    input.size = 1;
    input.onkeydown = (key) => (+key.key > 0 && +key.key <= 9) && input.value.length == 0 || key.key == 'ArrowUp' && +input.value < 9 || key.key == 'ArrowDown'|| key.key == 'Backspace';
    cell.appendChild(input)
    input_grid.appendChild(cell);
  }
  var solution_grid = document.getElementById('solution-grid');
  for (var j = 0; j < 81; j++) {
      const cell = document.createElement('div');
      if ((j % 27 > 17) && (j % 27 < 27)) {
        cell.classList.add("cell-border-bottom");
      }
      cell.classList.add('cell');
      solution_grid.appendChild(cell);
  }
}

async function readInput() {
  const cells = document.querySelectorAll('#input-grid > .cell');
  cells.forEach((cell, index) => {
    const cell_input = <HTMLInputElement>cell.childNodes[0]
    const value: number = parseInt(cell_input.value) || 0;

    gridData[index] = value
  });
  const solved = wasmModule.solve(gridData);
  if (solved) {
    displaySolution();
  } else {
    alert("Invalid input. Please check again.")
  }
}

function displaySolution() {
  const solutionGrid = document.getElementById('solution-grid');
  const cells = solutionGrid.childNodes;
  

  for (let i = 0; i < 9; i++) {
      for (let j = 0; j < 9; j++) {
        const cell = cells[i * 9 + j];
        cell.textContent = gridData[i * 9 + j].toString();
      }
  }
}

// Import the Rust module and initialize it
createGrid()
const btn = document.getElementById("solve-button");

btn.addEventListener("click", function () {
  readInput()
});
