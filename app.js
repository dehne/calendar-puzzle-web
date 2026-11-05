// SPDX-License-Identifier: MIT
const status = document.getElementById("status");
const grid = document.getElementById("solution");
const form = document.getElementById("date-form");
const controls = document.getElementById("date-controls");
const weekday = document.getElementById("weekday");
const month = document.getElementById("month");
const day = document.getElementById("day");
const button = document.getElementById("solve");
let solver;

function setStatus(state, message) {
  status.textContent = message;
  status.dataset.state = state;
}

function showError(error) {
  grid.replaceChildren();
  grid.removeAttribute("aria-label");
  setStatus("error", `Error: ${error instanceof Error ? error.message : String(error)}`);
  console.error(error);
}

function fillOptions(select, values) {
  select.replaceChildren(...values.map((value) => {
    const option = document.createElement("option");
    option.value = String(value);
    option.textContent = String(value);
    return option;
  }));
}

function updateDays(selectedDay = Number(day.value)) {
  const lastDay = solver.days_in_month(month.value);
  fillOptions(day, Array.from({ length: lastDay }, (_, i) => i + 1));
  day.value = String(Math.min(Math.max(selectedDay, 1), lastDay));
}

function selectionChanged() {
  grid.replaceChildren();
  grid.removeAttribute("aria-label");
  setStatus("ready", "Date changed. Click Solve to find its arrangement.");
}

function render(cells) {
  if (cells.length !== 63) throw new Error("The solver returned an invalid board length.");
  const fragment = document.createDocumentFragment();
  for (const label of cells) {
    const cell = document.createElement("div");
    const isPiece = label.length === 1 && "FILPNTUVWXYZ".includes(label);
    cell.className = isPiece ? `cell piece-${label}` : "cell excluded";
    cell.textContent = label;
    fragment.append(cell);
  }
  grid.replaceChildren(fragment);
  grid.setAttribute("aria-label", `Solution for ${weekday.value}, ${month.value} ${day.value}: 9 columns by 7 rows`);
}

form.addEventListener("submit", (event) => {
  event.preventDefault();
  if (!solver || button.disabled) return;
  button.disabled = true;
  grid.replaceChildren();
  grid.removeAttribute("aria-label");
  let result;
  try {
    const start = performance.now();
    result = solver.solve(weekday.value, month.value, day.value);
    const cells = result.cells;
    const solved = result.solved;
    const nodes = result.nodes_visited;
    const elapsed = performance.now() - start;
    const diagnostics = `${nodes.toLocaleString()} search nodes · ${elapsed.toFixed(2)} ms`;
    if (solved) {
      render(cells);
      setStatus("success", `SOLVED · ${weekday.value}, ${month.value} ${day.value} · ${diagnostics}`);
    } else {
      setStatus("unsolvable", `UNSOLVABLE · ${weekday.value}, ${month.value} ${day.value} · ${diagnostics}`);
    }
  } catch (error) {
    showError(error);
  } finally {
    result?.free();
    button.disabled = false;
  }
});

try {
  // Keep import and fetch inside the try block so initialization failures are visible.
  const { default: init, CalendarSolver } = await import("./pkg/calendar_puzzle_web.js");
  await init();
  const response = await fetch("./layouts/two-swaps.txt");
  if (!response.ok) throw new Error(`Could not load calendar layout: HTTP ${response.status}`);
  solver = new CalendarSolver(await response.text());
  fillOptions(weekday, solver.weekdays());
  fillOptions(month, solver.months());
  const today = new Date();
  weekday.selectedIndex = today.getDay();
  month.selectedIndex = today.getMonth();
  updateDays(today.getDate());
  month.addEventListener("change", () => {
    try {
      updateDays();
      selectionChanged();
    } catch (error) { showError(error); }
  });
  weekday.addEventListener("change", selectionChanged);
  day.addEventListener("change", selectionChanged);
  controls.disabled = false;
  button.disabled = false;
  setStatus("ready", "Choose a date, then click Solve.");
} catch (error) {
  solver?.free();
  solver = undefined;
  showError(error);
}
