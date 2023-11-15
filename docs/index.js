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
                ballContainer.style.backgroundColor = COLORS[board[i][j].player];
                for (let b = 1; b <= atoms; b++) {
                    const ball = ballTemplate.cloneNode(true);
                    if (atoms <= 4) {
                        ball.classList.add(`ball-${atoms}-${b}`);
                    } else {
                        ball.classList.add(`ball-any-any`);
                    }
                    ballContainer.appendChild(ball);
                }
                ballContainer.classList.remove("rotate-left");
                ballContainer.classList.remove("rotate-right");
                if (atoms === 2) {
                    ballContainer.classList.add("rotate-left");
                } else if (atoms === 3) {
                    ballContainer.classList.add("rotate-right");
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
    const [_id, i, j] = /cell-(\d+)-(\d+)/.exec(this.id);
    const state = JSON.parse(getState());
    const turn = state["turn"];
    const height = state["board"].length;
    const width = state["board"][0].length;
    const explosions = JSON.parse(addAtom(parseInt(i), parseInt(j))) || [];
    const near = {
        "up": [-1, 0],
        "right": [0, 1],
        "down": [1, 0],
        "left": [0, -1],
    }
    for (const round of explosions) {
        for (const {coord: [i, j], atoms} of round) {
            const balls = document.getElementById(`ball-container-${i}-${j}`);
            for (let i = 0; i < atoms; i++) {
                try {
                    balls.removeChild(balls.lastChild);
                } catch {
                }
            }
            let directions = [];
            switch (atoms) {
                case 2:
                    if (i === 0) {
                        directions.push("down");
                    } else {
                        directions.push("up");
                    }
                    if (j === 0) {
                        directions.push("right");
                    } else {
                        directions.push("left");
                    }
                    break;
                case 3:
                    if (i === 0) {
                        directions.push("down");
                        directions.push("left");
                        directions.push("right");
                    } else if (j === 0) {
                        directions.push("down");
                        directions.push("right");
                        directions.push("up");
                    } else if (i === height - 1) {
                        directions.push("left");
                        directions.push("right");
                        directions.push("up");
                    } else {
                        directions.push("left");
                        directions.push("down");
                        directions.push("up");
                    }
                    break;
                case 4:
                    directions = ["up", "down", "left", "right"];
                    break;
            }
            const ballTemplate = document.createElement("div");
            ballTemplate.classList.add("ball");
            for (const direction of directions) {
                const ball = ballTemplate.cloneNode();
                ball.classList.add(direction);
                const next = document.getElementById(`ball-animator-${i + near[direction][0]}-${j + near[direction][1]}`);
                next.appendChild(ball);
                next.style.backgroundColor = COLORS[turn];
            }
        }
        await new Promise(r => setTimeout(r, 200));
        for (let i = 0; i < height; i++) {
            for (let j = 0; j < width; j++) {
                const container = document.getElementById(`ball-container-${i}-${j}`);
                const animator = document.getElementById(`ball-animator-${i}-${j}`);
                for (const child of animator.childNodes) {
                    animator.removeChild(child);
                    container.appendChild(child);
                }
            }
        }
    }
    render();
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

function sleep(ms) {
    console.log("slee");
    (new Promise(resolve => setTimeout(resolve, ms)).then(_ => {
    }));
    console.log("sleep");
}
