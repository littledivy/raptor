# `raptor`

Experimental asynchronous serverless engine powered by Deno.

```typescript
// hello_world.ts
import { Request } from "https://deno.land/x/raptor/mod.ts";

export function handle(request: Request) {
    request.send({ body: "Hello World" })
}
```

```
raptord ./hello_world.ts

Loaded serverless function from ./hello_world.ts
Listening at http://120.0.0.1:3000/
```