### Curl command ###

```
curl -X OPTIONS localhost:8083/questions -H "Access-Control-Request-Method: PUT" -H "Access-Control-Request-Headers: content-type" -H "Origin: https://not-origin.io" --verbose
```