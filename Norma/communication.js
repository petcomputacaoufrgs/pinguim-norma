import init, {bar} from "./norma/pkg/norma"

export function foo() {
    console.log("bar")
}

const baz = async () => {
    await init()
        .then(() => {
            bar();
    });
}
