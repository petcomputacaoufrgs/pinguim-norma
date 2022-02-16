import {
    init,
    loadCode,
    saveCode,
    saveCodeHist,
    loadCodeHist
} from './common.js';
import { Editor, Highlighter } from 'pinguim-editor';
import * as wasm from "norma-wasm";

const editor = new Editor({
    targetTextArea: document.getElementById('userinput'),
    targetPre: document.getElementById('codeediting'),
    currLineSpan: document.getElementById('input-line'),
    currColumnSpan: document.getElementById('input-column'),
    saveCode,
    loadCode,
    saveCodeHist,
    loadCodeHist,
    handleKey: evt => {
        if (evt.key == 'Enter') {
            if (editor.isBetweenCurlies()) {
                evt.preventDefault();
                editor.edit('\n    \n');
                editor.changeSelection(
                    editor.selectionStart - 1,
                    editor.selectionEnd - 1,
                );
            }
        }
    },
    highlighter: new Highlighter(
        {
            className: 'comment',
            regex: /\/\/.*\n/
        },
        {
            className: 'reserved',
            regex: /\bmain\b|\bif\b|\bthen\b|\belse\b|\bdo\b|\bgoto\b|\boperation\b|\btest\b/
        },
        {
            className: 'label',
            regex: /[a-zA-z0-9_-]*[:]/
        },
        {
            className: 'builtin',
            regex: /\binc\b|\bdec\b|\bzero\b|\badd\b|\bsub\b|\bcmp\b/
        },
        {
            className: 'punctuation',
            bracket: { name: 'parens', direction: 'opening' },
            regex: /\(/,
        },
        {
            className: 'punctuation',
            bracket: { name: 'parens', direction: 'closing' },
            regex: /\)/,
        },
        {
            className: 'punctuation',
            bracket: { name: 'curly-brackets', direction: 'opening' },
            regex: /\{/,
        },
        {
            className: 'punctuation',
            bracket: { name: 'curly-brackets', direction: 'closing' },
            regex: /\}/,
        },
    ),
});

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

    reader.onload = evt => {
        editor.content = evt.target.result;
    };
}

actualBtn.addEventListener('change', () => upload(actualBtn.files[actualBtn.files.length - 1]));
downloadBtn.addEventListener('click', evt => {
    evt.preventDefault();
    download(editor.content, "maqnorma.mn");
});

// Log area
const logAreaText = document.getElementById('log-area__text');

const toggleLogColor = (correct) => {
    if (correct) {
        logAreaText.classList.remove("log-area__errors");
        logAreaText.classList.add("log-area__corrects");
    } else {
        logAreaText.classList.add("log-area__errors");
        logAreaText.classList.remove("log-area__corrects");
    }
}

init(() => {
    editor.load();
});

//---------- WASM ==========
init(() => {
    let interpreter = null;

    //---------- VERIFICAR CÓDIGO  ==========
    document.getElementById('verify').onclick = () => {
        interpreter = null;

        if (editor.content == '') {
            logAreaText.textContent = 'Entrada vazia!';
            toggleLogColor(false);
        } else {
            try {
                wasm.check(editor.content);
                logAreaText.textContent = 'Código OK!';
                toggleLogColor(true);
            } catch (errors) {
                logAreaText.textContent = '';
                let first = true;
                for (const error of errors) {
                    if (first) {
                        first = false;
                    } else {
                        logAreaText.textContent += '\n\n\n';
                    }
                    logAreaText.textContent += 'ERRO: ' + error.span.rendered;
                    logAreaText.textContent += '\n\n' + error.message;
                }
                toggleLogColor(false);
            }
        }
    };
});
