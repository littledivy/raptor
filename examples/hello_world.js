onmessage = function (e) {
    console.log("Handler called.")
    console.log(e.data)
    postMessage("Done");
}
