import init, {newGame} from "./pkg/frontend.js";

init().then(initializePage);

const FORCE_MOBILE = true;
const SMALL_WIDTH = 6, SMALL_HEIGHT = 11;
const BIG_WIDTH = 10, BIG_HEIGHT = 18;
const COLORS = ["red", "blue", "green", "yellow", "purple", "orange", "cyan", "magenta"];

function initializePage() {
    let device = "desktop";
    if (/Android|webOS|iPhone|iPad|iPod|BlackBerry|IEMobile|Opera Mini/i.test(navigator.userAgent) || FORCE_MOBILE) {
        device = "mobile";
    }
    document.getElementById("game-container").classList.add(device);
    initializePlayersSelector();
    //todo togliere
    initializeGrid(false);
    render();
}

function initializePlayersSelector() {
    const playersSelect = document.getElementById("players-select");
    for (let i = 2; i <= 8; i++) {
        const option = document.createElement("option");
        option.value = `${i}`;
        option.innerText = `${i}`;
        playersSelect.appendChild(option);
    }
}

function initializeGrid(large) {
    const grid = document.getElementById("grid");
    grid.innerHTML = "";
    grid.classList.remove("big", "small");
    let width;
    let height;
    if (large) {
        width = BIG_WIDTH;
        height = BIG_HEIGHT;
        grid.classList.add("big");
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
            row.appendChild(cellTemplate.cloneNode(false));
        }
        grid.appendChild(row);
    }
}

function render() {

}

function feNewGame() {
    const grid_size_large = document.getElementById("grid-size").checked;
    initializeGrid(grid_size_large);
}


document.getElementById("start-button").addEventListener("click", feNewGame);
