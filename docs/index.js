import init, {newGame, addAtom, getState} from "./pkg/frontend.js";

init().then(r => {
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
    const ballAnimatorTemplate = document.createElement("div");
    ballAnimatorTemplate.classList.add("ball-animator");
    for (let i = 0; i < height; i++) {
        const row = rowTemplate.cloneNode(false);
        for (let j = 0; j < width; j++) {
            const cell = cellTemplate.cloneNode(false);
            cell.id = `cell-${i}-${j}`;
            const ballContainer = ballContainerTemplate.cloneNode(false);
            ballContainer.id = `ball-container-${i}-${j}`;
            cell.appendChild(ballContainer);
            const ballAnimator = ballAnimatorTemplate.cloneNode(false);
            ballAnimator.id = `ball-animator-${i}-${j}`;
            cell.appendChild(ballAnimator);
            cell.addEventListener("click", feAddAtom);
            row.appendChild(cell);
        }
        grid.appendChild(row);
    }
    const {board, turn} = JSON.parse(getState());
    render(board, turn);
}

function render(board, turn) {
    for (let i = 0; i < board.length; i++) {
        for (let j = 0; j < board[i].length; j++) {
            document.getElementById(`cell-${i}-${j}`).style.borderColor = COLORS[turn];
            const ballContainer = document.getElementById(`ball-container-${i}-${j}`);
            ballContainer.innerHTML = "";
            ballContainer.className = "";
            const atoms = board[i][j].atoms;
            ballContainer.classList.add(`ball-container`);
            ballContainer.classList.add(`ball-container-${atoms}`);
            if (atoms > 0) {
                const ballTemplate = document.createElement("div");
                ballTemplate.classList.add("ball");
                ballContainer.style.backgroundColor = COLORS[board[i][j].player];
                for (let b = 1; b <= atoms; b++) {
                    ballContainer.appendChild(ballTemplate.cloneNode(false));
                }
                if (atoms > 1) {
                    if (atoms % 2 === 0) {
                        ballContainer.classList.add("rotate-left");
                    } else {
                        ballContainer.classList.add("rotate-right");
                    }
                }
            }
        }
    }
}

let ANIMATING = false;

async function feAddAtom() {
    if (ANIMATING) {
        return;
    }
    ANIMATING = true;
    const cellId = this.id.split("-");
    const i = parseInt(cellId[1]);
    const j = parseInt(cellId[2]);
    const state = JSON.parse(getState());
    const turn = state["turn"];
    // const height = state["board"].length;
    // const width = state["board"][0].length;
    const explosions = JSON.parse(addAtom(i, j)) || [];
    for (const {result, exploded} of explosions) {
        render(result, turn);
        await sleep(200);
    }
    const {board, turn: next_turn} = JSON.parse(getState());
    render(board, next_turn);
    ANIMATING = false;
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
// document.getElementById("undo").addEventListener("click", feUndo);
document.getElementById("restart").addEventListener("click", feNewGame);
document.getElementById("back-to-menu").addEventListener("click", backToMenu);
document.getElementById("players-slider").oninput = refreshPlayersCount;

async function sleep(ms) {
    await new Promise(r => setTimeout(r, 200));
}
