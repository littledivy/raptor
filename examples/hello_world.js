// import { handle } from "FILE";

onmessage = function (e) {
    const request = {
        ...e.data,
        send: (response) => {
            postMessage({ type: "send", response });
        },
        redirect: (response) => {
            postMessage({ type: "send", response })
        },
    }
    
    handle(request);
}

function handle(request) {
    console.log(request);
    request.send("Hello");
}

