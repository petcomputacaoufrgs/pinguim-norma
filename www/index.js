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

    reader.onload = evt => {
        textAreaHTML.value = evt.target.result;
        highlight();
    }
}

actualBtn.addEventListener('change', () => upload(actualBtn.files[actualBtn.files.length - 1]));
downloadBtn.addEventListener('click', evt => {
    evt.preventDefault();
    download(textAreaHTML.value, "maqnorma.mn");
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

// Local Storage
const getLastCode = () => {
    textAreaHTML.value = getStorage();
    highlight();
    syncScroll();
};

// Highlight
const textAreaHTML = document.getElementById('userinput');
const preAreaHTML = document.getElementById('codeediting');

class Highlighter {
    constructor(...types) {
        this.types = types;

        const alternatives = types.map(type => '(' + type.regex.source + ')');
        const flags = types.reduce(
            (flags, type) => {
                for (const flag of type.regex.flags) {
                    if (flags.indexOf(flag) < 0) {
                        flags += flag;
                    }
                }
                return flags;
            },
            ''
        );

        this.splitRegex = new RegExp(alternatives.join('|'), flags);
    }

    highlight(inputElement, targetElement) {
        const baseText = inputElement.value;
        const brackets = {};
        let index = 0;

        targetElement.innerHTML = '';

        for (let piece of baseText.split(this.splitRegex)) {
            piece = piece || "";

            const type = this.types.find(type => type.regex.test(piece));

            let child;
            if (type === undefined) {
                child = document.createTextNode(piece);
            } else {
                child = document.createElement('span');
                child.setAttribute('class', type.className);
                child.textContent = piece;

                if (type.bracket !== undefined) {
                    this.handleBracket(
                        inputElement,
                        piece,
                        type,
                        brackets,
                        index,
                        child,
                    );
                }
            }

            targetElement.appendChild(child);
            index += piece.length;
        }

        targetElement.appendChild(document.createElement('br'));
    }

    handleBracket(inputElement, piece, type, brackets, index, child) {
        let isSelected = (
            inputElement.selectionStart == index
            && inputElement.selectionEnd <= index + piece.length
        );
        const name = type.bracket.name;
        brackets[name] = brackets[name] || [];

        switch (type.bracket.direction) {
            case 'opening': {
                brackets[name].push({ node: child, selected: isSelected });
                break;
            }
            case 'closing': {
                const prev = brackets[name].pop();
                if (prev !== undefined && (prev.selected || isSelected)) {
                    let cls = child.getAttribute('class');
                    child.setAttribute('class', cls + ' selected-bracket');

                    cls = prev.node.getAttribute('class');
                    prev.node.setAttribute( 'class', cls + ' selected-bracket');
                }

                break;
            }
        }
    }
}


const highlighter = new Highlighter(
    { className: 'comment', regex: /\/\/.*\n/ },
    { className: 'reserved', regex: /\bmain\b|\bif\b|\bthen\b|\belse\b|\bdo\b|\bgoto\b|\boperation\b|\btest\b/ },
    { className: 'label', regex: /[a-zA-z0-9_-]*[:]/ },
    { className: 'builtin', regex: /\binc\b|\bdec\b|\bzero\b|\badd\b|\bsub\b|\bcmp\b/ },
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
);

const highlight = () => {
    highlighter.highlight(textAreaHTML, preAreaHTML);
    setStorage(textAreaHTML.value);
    syncScroll();
};

const handleKeys = {
    'Tab': evt => handleTab(evt),
    'Enter': evt => handleEnter(evt),
    'Backspace': evt => handleBackspace(evt),
    '(': evt => handleBracket(evt),
    '{': evt => handleCurly(evt)
};

textAreaHTML.addEventListener('keyup', evt => highlight());

textAreaHTML.addEventListener('keydown', evt => {
    if (evt.key in handleKeys) {
        handleKeys[evt.key](evt);
    }
});

textAreaHTML.addEventListener('scroll', evt => syncScroll());

textAreaHTML.addEventListener('input', evt => highlight());

textAreaHTML.addEventListener('click', evt => highlight());

const syncScroll = () => {
    preAreaHTML.scrollTop = textAreaHTML.scrollTop;
};

const handleTab = evt => {
    evt.preventDefault();
    const start = textAreaHTML.selectionStart;
    const end = textAreaHTML.selectionEnd;

    textAreaHTML.value = textAreaHTML.value.substring(0, start) +
        `    ` + textAreaHTML.value.substring(end);

    textAreaHTML.selectionStart = textAreaHTML.selectionEnd = start + 4;
};

const handleEnter = evt => {
    const start = textAreaHTML.selectionStart;
    const end = textAreaHTML.selectionEnd;

    if((textAreaHTML.value[textAreaHTML.selectionStart - 1] == '{') &&
        (textAreaHTML.value[textAreaHTML.selectionStart] == '}')) {
        evt.preventDefault();
        const start = textAreaHTML.selectionStart;
        const end = textAreaHTML.selectionEnd;

        textAreaHTML.value = textAreaHTML.value.substring(0, start) +
            "\n" + `    ` + "\n" + textAreaHTML.value.substring(end);

        textAreaHTML.selectionStart = textAreaHTML.selectionEnd = start + 5;
    }
};

const handleBackspace = evt => {
    const start = textAreaHTML.selectionStart;
    const end = textAreaHTML.selectionEnd;

    if(((textAreaHTML.value[textAreaHTML.selectionStart - 1] == '(') &&
        (textAreaHTML.value[textAreaHTML.selectionStart] == ')'))
        ||
        ((textAreaHTML.value[textAreaHTML.selectionStart - 1] == '{') &&
        (textAreaHTML.value[textAreaHTML.selectionStart] == '}'))) {

        evt.preventDefault();

        textAreaHTML.value = textAreaHTML.value.substring(0, start - 1)
            + textAreaHTML.value.substring(end + 1);

        textAreaHTML.selectionStart = textAreaHTML.selectionEnd = start - 1;
    }
};


const handleBracket = evt => {
    evt.preventDefault();
    const start = textAreaHTML.selectionStart;
    const end = textAreaHTML.selectionEnd;

    textAreaHTML.value = textAreaHTML.value.substring(0, start) +
        "()" + textAreaHTML.value.substring(end);

    textAreaHTML.selectionStart = textAreaHTML.selectionEnd = end + 1;
};

const handleCurly = evt => {
    evt.preventDefault();
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

//---------- WASM ==========
init(() => {
    let interpreter = null;

    const source = () => document.getElementById('userinput').value;

    //---------- VERIFICAR CÓDIGO  ==========
    document.getElementById('verify').onclick = () => {
        interpreter = null;

        if (textAreaHTML.value == '') {
            logAreaText.textContent = 'Entrada vazia!';
            toggleLogColor(false);
        } else {
            try {
                wasm.check(source());
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
