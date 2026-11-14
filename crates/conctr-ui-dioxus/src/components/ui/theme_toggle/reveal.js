// Runs only in response to activation, never during SSR. The framework remains
// the owner of theme state; this script only coordinates before/after snapshots.
const [x, y, nextDark] = await dioxus.recv();
const root = document.documentElement;
const marker = 'data-cn-theme-reveal';
if (root.hasAttribute(marker)) {
    dioxus.send(false);
    return;
}
const trigger = document.elementFromPoint(x, y)?.closest('button');
const icon = trigger?.querySelector('.cn-theme-toggle');
if (!document.startViewTransition || !icon ||
    matchMedia('(prefers-reduced-motion: reduce)').matches) {
    dioxus.send(true);
    dioxus.send(true);
    return;
}
// These document pseudo-elements cannot be scoped by the CSS-module parser.
// Install their rules only for the duration of this component's transition.
const style = document.createElement('style');
style.textContent = `
html[data-cn-theme-reveal]::view-transition-old(root),
html[data-cn-theme-reveal]::view-transition-new(root) {
    animation: none; mix-blend-mode: normal;
}
html[data-cn-theme-reveal]::view-transition-old(root) { z-index: 1; }
html[data-cn-theme-reveal]::view-transition-new(root) { z-index: 2; }
`;
document.head.append(style);
root.setAttribute(marker, '');
let transition;
let observer;
let timer;
let requested = false;
try {
    transition = document.startViewTransition(() => new Promise(resolve => {
        const finish = () => {
            observer.disconnect();
            clearTimeout(timer);
            resolve();
        };
        observer = new MutationObserver(() => {
            if (icon.dataset.dark === String(nextDark) || !icon.isConnected) finish();
        });
        observer.observe(icon, { attributes: true, attributeFilter: ['data-dark'] });
        // A controlled caller may ignore the request or unmount. Never leave
        // rendering frozen while waiting for an update that will not arrive.
        timer = setTimeout(() => { transition.skipTransition(); finish(); }, 1500);
        requested = true;
        dioxus.send(true);
    }));
    await transition.ready;
    const radius = Math.hypot(Math.max(x, innerWidth - x), Math.max(y, innerHeight - y));
    const circle = [`circle(0px at ${x}px ${y}px)`, `circle(${radius}px at ${x}px ${y}px)`];
    const animation = root.animate({
        clipPath: circle
    }, {
        duration: 450,
        easing: 'cubic-bezier(0.22, 1, 0.36, 1)',
        fill: 'forwards',
        pseudoElement: '::view-transition-new(root)'
    });
    await animation.finished;
} catch {
    // Skipped transitions, unsupported pseudo-element animations and navigation
    // must still apply exactly one state update and release the snapshot.
    if (!requested) { requested = true; dioxus.send(true); }
    transition?.skipTransition();
} finally {
    clearTimeout(timer);
    observer?.disconnect();
    if (transition) await transition.finished.catch(() => {});
    root.removeAttribute(marker);
    style.remove();
    dioxus.send(true);
}
