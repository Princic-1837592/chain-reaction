import init, {} from "./pkg/frontend.js";

init().then(initializePage);

const FORCE_MOBILE = true;
const SMALL_WIDTH = 6, SMALL_HEIGHT = 11;
const BIG_WIDTH = 10, BIG_HEIGHT = 18;

function initializePage() {
    let device = "desktop";
    if (/Android|webOS|iPhone|iPad|iPod|BlackBerry|IEMobile|Opera Mini/i.test(navigator.userAgent) || FORCE_MOBILE) {
        device = "mobile";
    }
    document.getElementById("game-container").classList.add(device);
    //todo togliere
    initializeGrid("small");
    render();
}

function initializeGrid(type) {
    const grid = document.getElementById("grid");
    let width;
    let height;
    if (type === "small") {
        width = SMALL_WIDTH;
        height = SMALL_HEIGHT;
        grid.classList.add("small");
    } else {
        width = BIG_WIDTH;
        height = BIG_HEIGHT;
        grid.classList.add("big");
    }
    const row_template = document.createElement("div");
    row_template.classList.add("row");
    const cell_template = document.createElement("div");
    cell_template.classList.add("cell")
    for (let i = 0; i < height; i++) {
        const row = row_template.cloneNode(false);
        for (let j = 0; j < width; j++) {
            row.appendChild(cell_template.cloneNode(false));
        }
        grid.appendChild(row);
    }
}

function render() {
    
}
