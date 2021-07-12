import init, {compileText, DataExporter, getValue} from './norma/pkg/norma.js';

let machinePtr;

export const send_text = async text => {
    await init()
            .then( () => {
                machinePtr = compileText(text);
                console.log(machinePtr)
                console.log(machinePtr.getLines())
                console.log(machinePtr.getInterpreter())
                console.log()
            });
}

document.getElementById('verify').addEventListener('click', () => send_text("abc123"));