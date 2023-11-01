import init, {} from "./pkg/frontend.js";

init().then(initializePage);

const FORCE_MOBILE = false;

function initializePage() {
    let dir = "desktop";
    if (/Android|webOS|iPhone|iPad|iPod|BlackBerry|IEMobile|Opera Mini/i.test(navigator.userAgent) || FORCE_MOBILE) {
        dir = "mobile";
    }
    for (const style of ["big", "selection", "small"]) {
        const css = document.createElement("link");
        css.rel = "stylesheet";
        css.href = `${dir}/${style}.css`
        document.getElementsByTagName("body")[0].appendChild(css);
    }
}

function initializeGrid() {
    const container = document.getElementById("grid-container");
    const width = 6, height = 8;
    const cell = document.createElement("div");
    cell.classList.add("cell")
}
