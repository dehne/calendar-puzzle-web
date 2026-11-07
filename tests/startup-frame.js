// SPDX-License-Identifier: MIT
// Test-only instrumentation; the production module and real Wasm solver are used.
const scenario = JSON.parse(frameElement.dataset.scenario);
const NativeDate = Date;
window.Date = class extends NativeDate {
  constructor(...args) { super(...(args.length ? args : scenario.date)); }
  getUTCDay() { throw new Error("Startup must use the local weekday"); }
  getUTCMonth() { throw new Error("Startup must use the local month"); }
  getUTCDate() { throw new Error("Startup must use the local day"); }
};
window.startupChecks = {
  calls: [],
  errors: [],
  initiallyDisabled: document.getElementById("solve").disabled && document.getElementById("date-controls").disabled,
};
const checks = window.startupChecks;
console.error = (error) => checks.errors.push(String(error));
const { CalendarSolver } = await import("../pkg/calendar_puzzle_web.js");
const originalSolve = CalendarSolver.prototype.solve;
CalendarSolver.prototype.solve = function (...args) {
  if (document.getElementById("date-controls").disabled) throw new Error("Solve before initialization completed");
  checks.calls.push(args);
  if (scenario.failure === "solve") throw new Error("Test solve failure");
  return originalSolve.apply(this, args);
};
const originalFetch = window.fetch;
window.fetch = async (resource, options) => {
  const path = String(resource);
  if (scenario.failure === "wasm" && path.endsWith(".wasm")) throw new Error("Test Wasm loading failure");
  if (path.endsWith("two-swaps.txt")) {
    if (scenario.failure === "fetch") return new Response("", { status: 404 });
    if (scenario.failure === "layout") return new Response("invalid calendar layout");
  }
  return originalFetch(resource, options);
};
try {
  await import("../app.js");
} finally {
  checks.complete = true;
}
