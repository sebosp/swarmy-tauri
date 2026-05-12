# Swarmy tauri/leptos frontend

# Development

## Dependencies
Tauri requirement plus tailwindcss and daisyui
```
$ curl -sLO https://github.com/tailwindlabs/tailwindcss/releases/download/v4.2.4/tailwindcss-linux-x64
$ vim README.md
$ chmod a+x tailwindcss-linux-x64
$ mv tailwindcss-linux-x64 tailwindcss
$ mv tailwindcss ~/local/bin/
$ curl -sLO https://github.com/dobicinaitis/tailwind-cli-extra/releases/download/v2.8.3/tailwindcss-extra-linux-x64
$ chmod a+x tailwindcss-extra-linux-x64
$ mv tailwindcss-extra-linux-x64 tailwindcss-extra
$ mv tailwindcss-extra ~/local/bin/
$ cd ~/swarmy-tauri;
$ npm install daisyui
```

There's a bit of a strange bug when installing tailwindcss / tailwindcss-extra in Trunk.toml, for some reason installing one tries to
I think I had to use something like
```toml
[tool]
tailwindcss = "2.7.0" # This is actually tailwindcss-extra version
```
Then `trunk build` downloads the tailwindcss-extra plugin version 2.7.0 :shrug:
And then **sometimes** it works?

# TODO

- Currently only one version can be running at a time because of the settings.json that is written on "scan" tab.
- "caches" should be stored in a global location to avoid double downloading for different snapshots.
