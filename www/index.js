import init, {greet} from "RustSnake";

init().then(_ => {
    greet("Mert");
    console.log("all ok")
})