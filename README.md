# Development

1. Install bun: https://bun.sh/docs/installation
2. Run the following command in the root of the project to start the tailwind CSS compiler:

```bash
bunx tailwindcss -i ./input.css -o ./assets/tailwind.css --watch
```

Run the following command in the root of the project to start the Dioxus dev server:

```bash
dx serve --hot-reload
```

- Open the browser to http://localhost:8080