const CACHE_NAME = 'calc00-v2';
const ASSETS = [
  './',
  './index.html',
  './calc00.js',
  './calc00_bg.wasm',
  './manifest.json',
  './icon-192.png',
  './icon-512.png'
];

self.addEventListener('install', (e) => {
  e.waitUntil(
    caches.open(CACHE_NAME).then((cache) => {
      return cache.addAll(ASSETS);
    })
  );
});

self.addEventListener('activate', (event) => {
  event.waitUntil((async () => {
    const keys = await caches.keys();
    await Promise.all(keys
      .filter((key) => key.startsWith('calc00-') && key !== CACHE_NAME)
      .map((key) => caches.delete(key)));
    await self.clients.claim();
  })());
});

self.addEventListener('fetch', (e) => {
  const url = new URL(e.request.url);
  if (e.request.method !== 'GET' ||
      url.origin !== self.location.origin ||
      !url.href.startsWith(self.registration.scope)) {
    return;
  }

  e.respondWith(
    caches.open(CACHE_NAME).then(async (cache) => {
      const response = await cache.match(e.request);
      if (response) return response;

      if (e.request.mode === 'navigate' &&
          (url.pathname === new URL('./', self.registration.scope).pathname ||
           url.pathname === new URL('index.html', self.registration.scope).pathname)) {
        return cache.match('./index.html');
      }

      return fetch(e.request);
    })
  );
});