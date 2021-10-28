import { init, getStorage, setStorage } from './common.js';
import * as wasm from "norma-wasm";

// Upload and download buttons
const downloadBtn = document.getElementById('download_button');
const actualBtn = document.getElementById('upload_button');
const fileChosen = document.getElementById('file-chosen');

const download = (text, filename) => {
    const element = document.createElement('a');
    element.setAttribute('href', 'data:text/plain;charset=utf-8,' + encodeURIComponent(text));
    element.setAttribute('download', filename); 

    element.style.display = 'none';

    document.body.appendChild(element);
    element.click();
    document.body.removeChild(element);
}

const upload = (file) => {
    const reader = new FileReader();
    
    fileChosen.textContent = file.name;
    reader.readAsText(file, "UTF-8");

    reader.onload = (e) => {
        textAreaHTML.value = e.target.result;
        highlight();
    } 
}

actualBtn.addEventListener('change', () => upload(actualBtn.files[actualBtn.files.length - 1]));
downloadBtn.addEventListener('click', (e) => {
    e.preventDefault();
    download(textAreaHTML.value, "maqnorma.mn");
});

// Local Storage

const getLastCode = () => {
    textAreaHTML.value = getStorage();
    highlight();
};

// Highlight 
const textAreaHTML = document.getElementById('userinput');
const codeAreaHTML = document.getElementById('codeholder');
const preAreaHTML = document.getElementById('codeediting');

const reservedWords = /\bmain\b|\bif\b|\bthen\b|\belse\b|\bdo\b|\bgoto\b|\boperation\b|\btest\b/gi;
const builtInFuncs = /\binc\b|\bdec\b|\bzero\b|\badd\b|\bsub\b|\bcmp\b/gi;
const regexLabels = /[a-zA-z0-9_-]*[:]/g;

const spanEnd = '</span>';
const spanLabels = '<span class="label">';
const spanReserved = '<span class="reserved">';
const spanBuiltIn = '<span class="builtin">';

const highlight = () => {
    let baseText = textAreaHTML.value;
    let finalText = baseText.replace(regexLabels, (match) => spanLabels + match + spanEnd);
    finalText = finalText.replace(reservedWords, (match) => spanReserved + match + spanEnd);
    finalText = finalText.replace(builtInFuncs, (match) => spanBuiltIn + match + spanEnd);

    codeAreaHTML.innerHTML = finalText;
    setStorage(baseText);
};

const handleKeys = {
    'Tab': (e) => handleTab(e),
    'Enter': (e) => handleEnter(e),
    'Backspace': (e) => handleBackspace(e),
    '(': (e) => handleBracket(e),
    '{': (e) => handleCurly(e)
};

textAreaHTML.addEventListener('keyup', (evt) => highlight());

textAreaHTML.addEventListener('keydown', (e) => {
     try { handleKeys[e.key](e) }
     catch(e) {}
});

textAreaHTML.addEventListener('scroll', (e) => handleScroll());

const handleScroll = () => {
    preAreaHTML.scrollTop = textAreaHTML.scrollTop;
    preAreaHTML.scrollLeft = textAreaHTML.scrollLeft;
}

const handleTab = (e) => {
    e.preventDefault();
    const start = textAreaHTML.selectionStart;
    const end = textAreaHTML.selectionEnd;

    textAreaHTML.value = textAreaHTML.value.substring(0, start) + 
        `    ` + textAreaHTML.value.substring(end);

    textAreaHTML.selectionStart = textAreaHTML.selectionEnd = start + 4;
};

const handleEnter = (e) => {
    const start = textAreaHTML.selectionStart;
    const end = textAreaHTML.selectionEnd;

    if((textAreaHTML.value[textAreaHTML.selectionStart - 1] == '{') && 
        (textAreaHTML.value[textAreaHTML.selectionStart] == '}')) {
        e.preventDefault();
        const start = textAreaHTML.selectionStart;
        const end = textAreaHTML.selectionEnd;

        textAreaHTML.value = textAreaHTML.value.substring(0, start) +
            "\n" + `    ` + "\n" + textAreaHTML.value.substring(end);

        textAreaHTML.selectionStart = textAreaHTML.selectionEnd = start + 5;
    }
};

const handleBackspace = (e) => {
    const start = textAreaHTML.selectionStart;
    const end = textAreaHTML.selectionEnd;

    if(((textAreaHTML.value[textAreaHTML.selectionStart - 1] == '(') && 
        (textAreaHTML.value[textAreaHTML.selectionStart] == ')')) 
        || 
        ((textAreaHTML.value[textAreaHTML.selectionStart - 1] == '{') && 
        (textAreaHTML.value[textAreaHTML.selectionStart] == '}'))) {
            
        e.preventDefault();

        textAreaHTML.value = textAreaHTML.value.substring(0, start).slice(0, start - 1)
            + textAreaHTML.value.substring(end).slice(1, end);

        textAreaHTML.selectionStart = textAreaHTML.selectionEnd = start - 1;
    }
};

const handleBracket = (e) => {
    e.preventDefault();
    const start = textAreaHTML.selectionStart;
    const end = textAreaHTML.selectionEnd;

    textAreaHTML.value = textAreaHTML.value.substring(0, start) + 
        "()" + textAreaHTML.value.substring(end);

    textAreaHTML.selectionStart = textAreaHTML.selectionEnd = end + 1;
};

const handleCurly = (e) => {
    e.preventDefault();
    const start = textAreaHTML.selectionStart;
    const end = textAreaHTML.selectionEnd;

    textAreaHTML.value = textAreaHTML.value.substring(0, start) + 
        "{}" + textAreaHTML.value.substring(end);

    textAreaHTML.selectionStart = textAreaHTML.selectionEnd = end + 1;
};

// Init
init(() => {
    getLastCode();
});

// BEGIN Gambiarra pra testar WASM
init(() => {
    let interpreter = null;
    let running = false;

    const source = () => document.getElementById('userinput').value;
    const registerX = () => document.getElementById('gambiarra-reg-x').value;

    document.getElementById('gambiarra-check').onclick = () => {
        interpreter = null;
        try {
            wasm.check(source());
            console.log('wasm.check ok!');
        } catch (error) {
            console.log(error);
            console.log('wasm.check failed!');
        }
    };

    document.getElementById('gambiarra-compile').onclick = () => {
        interpreter = null;
        try {
            interpreter = wasm.compile(source());
            console.log(interpreter);
            console.log('wasm.compile ok!');
        } catch (error) {
            console.log(error);
            console.log('wasm.compile failed!');
        }
    };

    document.getElementById('gambiarra-data').onclick = () => {
        console.log(interpreter.data());
        console.log('interpreter.data ok!');
    };

    document.getElementById('gambiarra-instructions').onclick = () => {
        console.log(interpreter.instructions());
        console.log('interpreter.instructions ok!');
    };

    document.getElementById('gambiarra-status').onclick = () => {
        console.log(interpreter.status());
        console.log('interpreter.status ok!');
    };

    document.getElementById('gambiarra-reset').onclick = () => {
        interpreter.reset();
        console.log('interpreter.reset ok!');
    };

    document.getElementById('gambiarra-input').onclick = () => {
        interpreter.input(registerX());
        console.log('interpreter.input ok!');
    };

    document.getElementById('gambiarra-run-step').onclick = () => {
        console.log(interpreter.runStep());
        console.log('interpreter.runStep ok!');
    };

    document.getElementById('gambiarra-run-steps').onclick = () => {
        console.log(interpreter.runSteps(10000));
        console.log('interpreter.runSteps ok!');
    };

    document.getElementById('gambiarra-run-all').onclick = () => {
        const then = performance.now();

        const tick = () => {
            if (running) {
                const status = interpreter.runSteps(10000);
                running = status.running;
                if (running) {
                    setTimeout(tick, 10);
                } else {
                    const end = performance.now();
                    console.log(status);
                    console.log('Ended "run all" in', end - then + 'ms');
                }
            } else {
                console.log('Ended "run all"');
            }
        };

        console.log('Running all...');

        running = true;
        interpreter.reset();
        interpreter.input(registerX());
        tick();
    };

    document.getElementById('gambiarra-abort').onclick = () => {
        running = false;
        console.log('Aborting...');
    };
});
// END Gambiarra pra testar WASM
