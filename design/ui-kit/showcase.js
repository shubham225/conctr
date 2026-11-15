(function () {
  var root = document.documentElement;
  var btn = document.getElementById('theme');
  if (btn) btn.addEventListener('click', function () {
    var dark = root.dataset.theme ? root.dataset.theme === 'dark' : matchMedia('(prefers-color-scheme: dark)').matches;
    root.dataset.theme = dark ? 'light' : 'dark';
  });
  document.addEventListener('click', function (e) {
    var t = e.target.closest('[data-toggle]');
    if (t) t.setAttribute('aria-pressed', t.getAttribute('aria-pressed') === 'true' ? 'false' : 'true');
    var rm = e.target.closest('.cn-tag__remove');
    if (rm) rm.closest('.cn-tag').remove();
    var day = e.target.closest('.cn-calendar__day');
    if (day && !day.disabled) { var g = day.closest('.cn-calendar__grid'); g.querySelectorAll('[aria-selected]').forEach(function (d) { d.removeAttribute('aria-selected'); }); day.setAttribute('aria-selected', 'true'); }
    var tab = e.target.closest('.cn-tabs--vertical .cn-tab');
    if (tab) { tab.parentElement.querySelectorAll('.cn-tab').forEach(function (x) { x.setAttribute('aria-selected', 'false'); }); tab.setAttribute('aria-selected', 'true'); }
    var tr = e.target.closest('.cn-table--selectable tbody tr');
    if (tr && e.target.closest('input')) tr.setAttribute('aria-selected', e.target.checked ? 'true' : 'false');
    var th = e.target.closest('th[aria-sort]');
    if (th) { var cur = th.getAttribute('aria-sort'); th.parentElement.querySelectorAll('th[aria-sort]').forEach(function (x) { x.setAttribute('aria-sort', 'none'); }); th.setAttribute('aria-sort', cur === 'ascending' ? 'descending' : 'ascending'); }
    var tg = e.target.closest('.cn-tree-toggle');
    if (tg) tg.setAttribute('aria-expanded', tg.getAttribute('aria-expanded') === 'true' ? 'false' : 'true');
  });
  document.querySelectorAll('.cn-slider').forEach(function (s) {
    function upd() { var p = (s.value - s.min) / (s.max - s.min) * 100; s.style.setProperty('--p', p + '%'); var o = s.closest('.cn-slider-field'); if (o) { var v = o.querySelector('[data-out]'); if (v) v.textContent = s.value + (s.dataset.unit || ''); } }
    s.addEventListener('input', upd); upd();
  });
})();
