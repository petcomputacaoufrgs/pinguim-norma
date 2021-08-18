import init, {compileText, DataExporter} from './norma/pkg/norma.js';

let machinePtr;

export const send_text = async text => {
    await init()
            .then( () => {
                machinePtr = compileText(text);
                console.log(compileText(text) instanceof DataExporter);
            });
}

document.getElementById('verify').addEventListener('click', () => send_text("abc123"));