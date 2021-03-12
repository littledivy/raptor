// Serverless function entry point. All requests go through here.
export function handle(request) {
    console.log(`Incoming request from ${request.headers.host}`);
    if(request.method == "GET") {
        request.send({ body: "What do you want?" });
    } else if(request.method == "POST") {
        request.send({ body: `Thanks for giving me ${request.body}` })
    }
}
