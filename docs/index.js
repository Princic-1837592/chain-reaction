import init, {newGame, addAtom, getState} from "./pkg/frontend.js";

init().then(r => {
});

const FORCE_MOBILE = true;
// const FORCE_MOBILE = false;
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
    cellTemplate.classList.add("cell")
    for (let i = 0; i < height; i++) {
        const row = rowTemplate.cloneNode(false);
        for (let j = 0; j < width; j++) {
            const cell = cellTemplate.cloneNode(false);
            cell.id = `${i}-${j}`;
            cell.addEventListener("click", feAddAtom);
            row.appendChild(cell);
        }
        grid.appendChild(row);
    }
}

function render() {
    const {atoms: _atoms, board, max_atoms: _max_atoms, players: _players, turn: turn} = JSON.parse(getState());
    const ballContainerTemplate = document.createElement("div");
    ballContainerTemplate.classList.add("ball-container");
    for (let i = 0; i < board.length; i++) {
        for (let j = 0; j < board[i].length; j++) {
            const cell = document.getElementById(`${i}-${j}`);
            cell.innerHTML = "";
            if (board[i][j].atoms > 0) {
                const ballContainer = ballContainerTemplate.cloneNode(false);
                if (board[i][j].atoms >= 2) {
                    ballContainer.classList.add("rotate");
                }
                cell.appendChild(ballContainer);
            }
        }
    }
}

function feAddAtom() {
    const [_id, i, j] = /(\d+)-(\d+)/.exec(this.id);
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
document.getElementById("back-to-menu").addEventListener("click", backToMenu)
document.getElementById("players-slider").oninput = refreshPlayersCount;
