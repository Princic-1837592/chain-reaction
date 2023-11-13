import init, {newGame} from "./pkg/frontend.js";

init().then(() => {
});

const FORCE_MOBILE = true;
// const FORCE_MOBILE = false;
const SMALL_WIDTH = 6, SMALL_HEIGHT = 11;
const BIG_WIDTH = 10, BIG_HEIGHT = 18;
const COLORS = ["red", "blue", "green", "yellow", "purple", "orange", "cyan", "magenta"];

function initializePage() {
    let device = "desktop";
    if (/Android|webOS|iPhone|iPad|iPod|BlackBerry|IEMobile|Opera Mini/i.test(navigator.userAgent) || FORCE_MOBILE) {
        device = "mobile";
    }
    document.getElementById("game-container").classList.add(device);
    //todo togliere
    initializeGrid(false);
    render();
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
