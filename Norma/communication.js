import init, {compileText, DataExporter} from './norma/pkg/norma.js';

let dataPointer;

export const send_text = async text => {
    await init()
            .then( () => {
                dataPointer = compileText(text);
                console.log(dataPointer);
            });
}

export const show_info = (dp) => {
    console.log(dp)
    console.log(dp.getLines());
}

document.getElementById('checkbox').addEventListener('click', () => send_text("abc123"));
document.getElementById('verify').addEventListener('click', () => show_info(dataPointer));