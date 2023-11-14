import init, {newGame, addAtom, getState} from "./pkg/frontend.js";

init().then(r => {
    feNewGame();
});

const SMALL_WIDTH = 6, SMALL_HEIGHT = 11;
const BIG_WIDTH = 10, BIG_HEIGHT = 18;
const COLORS = ["red", "blue", "green", "yellow", "purple", "orange", "cyan", "magenta"];

function initializeGrid(large) {
    const grid = document.getElementById("grid");
    grid.innerHTML = "";
    grid.classList.remove("large", "small");
    let width;
    let height;
    if (large) {
        width = BIG_WIDTH;
        height = BIG_HEIGHT;
        grid.classList.add("large");
    } else {
        width = SMALL_WIDTH;
        height = SMALL_HEIGHT;
        grid.classList.add("small");
    }
    const rowTemplate = document.createElement("div");
    rowTemplate.classList.add("row");
    const cellTemplate = document.createElement("div");
    cellTemplate.classList.add("cell");
    const ballContainerTemplate = document.createElement("div");
    ballContainerTemplate.classList.add("ball-container");
    cellTemplate.appendChild(ballContainerTemplate);
    for (let i = 0; i < height; i++) {
        const row = rowTemplate.cloneNode(false);
        for (let j = 0; j < width; j++) {
            const cell = cellTemplate.cloneNode(true);
            cell.id = `cell-${i}-${j}`;
            cell.firstChild.id = `ball-container-${i}-${j}`;
            cell.addEventListener("click", feAddAtom);
            row.appendChild(cell);
        }
        grid.appendChild(row);
    }
    render();
}

function render() {
    const {atoms: _atoms, board, max_atoms: _max_atoms, players: _players, turn: turn} = JSON.parse(getState());
    // console.log(JSON.parse(getState()));
    for (let i = 0; i < board.length; i++) {
        for (let j = 0; j < board[i].length; j++) {
            document.getElementById(`cell-${i}-${j}`).style.borderColor = COLORS[turn];
            const ballContainer = document.getElementById(`ball-container-${i}-${j}`);
            ballContainer.innerHTML = "";
            const atoms = board[i][j].atoms;
            if (atoms > 0) {
                const ballTemplate = document.createElement("div");
                ballTemplate.classList.add("ball");
                ballTemplate.style.backgroundColor = COLORS[board[i][j].player];
                for (let b = 1; b <= atoms; b++) {
                    const ball = ballTemplate.cloneNode(true);
                    ball.classList.add(`ball-${atoms}-${b}`);
                    ballContainer.appendChild(ball);
                }
                if (atoms === 2) {
                    ballContainer.classList.add("rotate-left");
                } else if (atoms === 3) {
                    ballContainer.classList.add("rotate-right");
                }else{
                    ballContainer.classList.remove("rotate-left");
                    ballContainer.classList.remove("rotate-right");
                }
            }
        }
    }
}

function feAddAtom() {
    const [_id, i, j] = /cell-(\d+)-(\d+)/.exec(this.id);
    const _explosions = JSON.parse(addAtom(parseInt(i), parseInt(j)) || "null");
    render();
}

function feNewGame() {
    const large = document.getElementById("grid-size").checked;
    const players = document.getElementById("players-slider").value;
    newGame(large, parseInt(players));
    initializeGrid(large);
    const grid = document.getElementById("game-container");
    grid.classList.remove("invisible");
    const menu = document.getElementById("menu");
    menu.classList.add("invisible");
}

function backToMenu() {
    const grid = document.getElementById("game-container");
    grid.classList.add("invisible");
    const menu = document.getElementById("menu");
    menu.classList.remove("invisible");
}

function refreshPlayersCount() {
    document.getElementById("players-value").innerText = this.value;
}


document.getElementById("start-button").addEventListener("click", feNewGame);
// document.getElementById("undo").addEventListener("click", backToMenu);
document.getElementById("restart").addEventListener("click", feNewGame);
document.getElementById("back-to-menu").addEventListener("click", backToMenu);
document.getElementById("players-slider").oninput = refreshPlayersCount;
