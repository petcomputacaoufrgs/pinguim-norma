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
