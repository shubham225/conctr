// SMIL owns interpolation. Pause the SVG clock without replacing its shapes
// or losing its position when the window is hidden or the user pauses motion.
const svg = document.querySelector("#sign-in-artwork svg");
if (!svg) return;
const preference = matchMedia("(prefers-reduced-motion: reduce)");
const narrow = matchMedia("(max-width: 46.25rem)");
let paused = false;
// Start translated groups on the document animation timeline as soon as they
// mount. Their positions, easing and phase offsets live with the illustration.
const translations = [];
for (const track of svg.querySelectorAll("animateTransform[begin]")) {
    const times = track.getAttribute("keyTimes").split(";").map(Number);
    const curves = (track.getAttribute("keySplines") || "").split(";");
    const frames = track.getAttribute("values").split(";").map((point, index) => {
        const [x, y] = point.trim().split(/\s+/);
        return {
            transform: `translate(${x}px, ${y}px)`,
            offset: times[index],
            easing: curves[index] ? `cubic-bezier(${curves[index].trim().split(/\s+/).join(",")})` : "linear",
        };
    });
    const animation = track.parentElement.animate(frames, {
        duration: 1000 * parseFloat(track.getAttribute("dur")),
        delay: 1000 * parseFloat(track.getAttribute("begin")),
        iterations: Infinity,
    });
    track.remove();
    if (preference.matches) animation.currentTime = 4500;
    translations.push(animation);
}
function sync() {
    const stop = paused || preference.matches || narrow.matches || document.hidden;
    if (stop) svg.pauseAnimations();
    else svg.unpauseAnimations();
    for (const animation of translations) {
        if (stop) animation.pause();
        else animation.play();
    }
}
svg.setCurrentTime(preference.matches ? 4.5 : 0);
sync();
narrow.addEventListener("change", sync);
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
    narrow.removeEventListener("change", sync);
    preference.removeEventListener("change", sync);
    document.removeEventListener("visibilitychange", sync);
}
