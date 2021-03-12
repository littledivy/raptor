# `raptor`

Experimental asynchronous serverless engine powered by Deno.

```typescript
import { Request } from "https://deno.land/x/raptor/mod.ts";
export function handle(request: Request) {
    request.send({ body: "Hello World" })
}
```

```bash
> raptor ./hello_world.ts
Listening on http://120.0.0.1:3000/
```

### Status

Work in progress. Feel free to get in touch on Discord `divy#8574` or via [email](mailto:dj.srivastava23@gmail.com).

All code is written in `crates/core/main.rs` will be refacted to be more flexible.

### License

MIT