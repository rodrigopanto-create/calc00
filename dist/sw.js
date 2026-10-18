const CACHE_NAME = 'calc00-v1';
const ASSETS = [
  './',
  './index.html',
  './calc00.js',
  './calc00_bg.wasm'
];

self.addEventListener('install', (e) => {
  e.waitUntil(
    caches.open(CACHE_NAME).then((cache) => {
      return cache.addAll(ASSETS);
    })
  );
});

self.addEventListener('fetch', (e) => {
  e.respondWith(
    caches.match(e.request).then((response) => {
      return response || fetch(e.request);
    })
  );
});