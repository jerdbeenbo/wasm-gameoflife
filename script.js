// Import the wasm module
import init, {
  wasm_bridge_init,
  wasm_bridge_update,
  add_cell,
  get_current_state,
} from "./pkg/wasm_gameoflife.js";

let canvas, ctx;
const cellSize = 4;

let lastSimulationTime = 0;
const simulationInterval = 1000 / 60; // Run simulation 60 times per second

function animate(currentTime) {
  // Only run simulation if enough time has passed AND not paused
  if (!isPaused && currentTime - lastSimulationTime >= simulationInterval) {
    const data = wasm_bridge_update();
    window.currentSimulationData = data;
    lastSimulationTime = currentTime;
  }

  // Always draw, even when paused (so you can see new cells being added)
  if (window.currentSimulationData) {
    ctx.clearRect(0, 0, canvas.width, canvas.height);

    for (
      let i = 0;
      i < window.currentSimulationData.active_particles.length;
      i++
    ) {
      const [row, col] = window.currentSimulationData.active_particles[i];
      const x = col * cellSize;
      const y = row * cellSize;

      ctx.fillStyle = "blue";
      ctx.fillRect(x, y, cellSize, cellSize);
    }
  }

  requestAnimationFrame(animate);
}

function setupMouseInput() {
  canvas.addEventListener("mousedown", handleMouse);
  canvas.addEventListener("mousemove", handleMouseMove);
}

let isMouseDown = false;
let isPaused = false;

function handleMouse(event) {
  isMouseDown = true;
  isPaused = true;
  addCellAtMouse(event);
}

function handleMouseMove(event) {
  if (isMouseDown) {
    addCellAtMouse(event);
  }
}

// Stop drawing when mouse is released
document.addEventListener("mouseup", () => {
  isMouseDown = false;
  isPaused = false;
});

function addCellAtMouse(event) {
  // Get mouse position relative to canvas
  const rect = canvas.getBoundingClientRect();
  const mouseX = event.clientX - rect.left;
  const mouseY = event.clientY - rect.top;

  // Convert pixel coordinates to grid coordinates
  const col = Math.floor(mouseX / cellSize);
  const row = Math.floor(mouseY / cellSize);

  console.log(`Adding sand at grid(${row}, ${col})`);

  // Call Rust function to add cell
  add_cell(row, col);

  // Get current state without advancing simulation
  if (isPaused) {
    const data = get_current_state();
    window.currentSimulationData = data;
  }
}

async function draw() {
  // Initialize everything from wasm before committing to the draw
  await init();

  wasm_bridge_init();
  canvas = document.getElementById("canvas");

  if (isPaused) {
    ctx.fillStyle = "rgba(255, 0, 0, 0.1)";
    ctx.fillRect(0, 0, canvas.width, canvas.height);

    ctx.fillStyle = "red";
    ctx.font = "20px Arial";
    ctx.fillText("PAUSED - Drawing Mode", 10, 30);
  }

  if (canvas.getContext) {
    //create an object with tooling for drawing on the canvas
    ctx = canvas.getContext("2d");

    canvas.width = 1200;
    canvas.height = 800;

    setupMouseInput();

    animate();
  }
}

//Listen for go-ahead signal from browser to start the simulation
window.addEventListener("load", draw);
