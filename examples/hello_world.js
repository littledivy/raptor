onmessage = function (e) {
    console.log("Handler called.")
    console.log(e)
    postMessage("Done");
}
