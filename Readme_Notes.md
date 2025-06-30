### Curl command ###

```
curl -X OPTIONS localhost:8083/questions -H "Access-Control-Request-Method: PUT" -H "Access-Control-Request-Headers: content-type" -H "Origin: https://not-origin.io" --verbose
```

### filters and body::json()
In code
[minimal_main.rs](src/minimal_http/minimal_main.rs#L20-L25),
The order of 
```text 
.and(warp::body::json()) 
``` 
and 
```text
.and(store_filter.clone())
```
must match the order of parameters in your handler function. Always align filter order with handler argument order.