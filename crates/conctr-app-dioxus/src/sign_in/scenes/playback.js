// SMIL owns interpolation. Pause the SVG clock without replacing its shapes
// or losing its position when the window is hidden or the user pauses motion.
const svg = document.querySelector("#sign-in-artwork svg");
if (!svg) return;
const preference = matchMedia("(prefers-reduced-motion: reduce)");
let paused = false;
function sync() {
    if (paused || preference.matches || document.hidden) svg.pauseAnimations();
    else svg.unpauseAnimations();
}
if (preference.matches) svg.setCurrentTime(4.5);
sync();
preference.addEventListener("change", sync);
document.addEventListener("visibilitychange", sync);
try {
    for (;;) {
        const command = await dioxus.recv();
        if (command === "dispose") break;
        paused = command === "pause";
        sync();
    }
} finally {
    preference.removeEventListener("change", sync);
    document.removeEventListener("visibilitychange", sync);
}
