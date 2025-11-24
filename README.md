# Swarmy tauri/leptos frontend

# Development

## Dependencies
Tauri requirement plus tailwindcss and daisyui
```
cd ~/swarmy-tauri;
npm install daisyui
```

There's a bit of a strange bug when installing tailwindcss / tailwindcss-extra in Trunk.toml, for some reason installing one tries to
I think I had to use something like
```toml
[tool]
tailwindcss = "2.7.0" # This is actually tailwindcss-extra version
```
Then `trunk build` downloads the tailwindcss-extra plugin version 2.7.0 :shrug:
And then **sometimes** it works?