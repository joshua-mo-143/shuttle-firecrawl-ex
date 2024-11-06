## Shuttle Firecrawl example
This repository shows how you can create and deploy a web service that uses Firecrawl, to Shuttle.

## Features
- Uses the Shuttle runtime for one-line deploys
- Includes a single endpoint at the base route (`/`) for POST requests (for scraping pages).

## How to use
Before using this, don't forget to add your Firecrawl API key to `Secrets.toml`. If you are unsure on how to go about this, you can copy from `Secrets.toml.example` into `Secrets.toml` and add your API key to the appropriate part.

To use, you can run this locally with `shuttle run` or deploy it with `shuttle deploy`.

Once done, you can use the following cURL command to scrape a webpage (note that this command assumes running locally):
```bash
curl localhost:8000 -H 'Content-Type: application/json' -d '{"url":"https://firecrawl.dev"}'
```
Note that you may want to pipe your results into a file as the resulting JSON can be quite large and difficult to read in standard terminal. To do this, you can run the following cURL command:
```bash
curl localhost:8000 -H 'Content-Type: application/json' -d '{"url":"https://firecrawl.dev"}' -o results.txt
```

## Troubleshooting
- Don't forget to check your Firecrawl API key if you are experiencing troubles with not being able to use the Firecrawl API.
- The Shuttle service locally runs at port 8000 by default. You can change this with `--port`.
