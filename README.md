# Dropshot with OTEL tracing

This is an example of how to build a dropshot http server
with built in otel tracing

## Running Jaeger

```bash
docker run --rm -d --name jaeger -p 4317:4317 -p 16686:16686 jaegertracing/all-in-one:latest
```
