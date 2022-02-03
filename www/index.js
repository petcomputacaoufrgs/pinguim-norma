import {
    init,
    loadCode,
    saveCode,
    saveCodeHist,
    loadCodeHist
} from './common.js';
import * as wasm from "norma-wasm";

class Editor {
    constructor(params) {
        this.targetTextArea = params.targetTextArea;
        this.targetPre = params.targetPre;
        this.currLineSpan = params.currLineSpan;
        this.currColumnSpan = params.currColumnSpan;
        this.highlighter = params.highlighter;
        this.saveCode = params.saveCode;
        this.loadCode = params.loadCode;
        this.saveCodeHist = params.saveCodeHist;
        this.loadCodeHist = params.loadCodeHist;
        this.history = [];

        this.targetTextArea.addEventListener('keyup', evt => {
            this.refreshContent();
        });

        this.targetTextArea.addEventListener('keydown', evt => {
            this.handleKey(evt);
        });

        this.targetTextArea.addEventListener('scroll', evt => {
            this.syncScroll();
        });

        this.targetTextArea.addEventListener('input', evt => {
            this.refreshContent();
        });

        this.targetTextArea.addEventListener('click', evt => {
            this.refreshContent();
        });
    }

    updateContent(content) {
        this.targetTextArea.value = content;
        this.refreshContent();
    }

    get content() {
        return this.targetTextArea.value;
    }

    addToHistory(...actions) {
        this.history.push(actions);
        this.saveHistory();
    }

    apply(...actions) {
        for (const action of actions) {
            const start = this.targetTextArea.selectionStart;
            const end = this.targetTextArea.selectionStart;
            switch (action.type) {
                case 'insert': {
                    const prev = this.targetTextArea.value.substring(0, start);
                    const next = this.targetTextArea.value.substring(end);
                    this.targetTextArea.value = prev + action.data + next;
                    const newPosition = start + action.data.length;
                    this.targetTextArea.selectionStart = newPosition;
                    this.targetTextArea.selectionEnd = newPosition;
                    break;
                }
                case 'delete': {
                    const prev = this.targetTextArea.value.substring(0, start);
                    const next = this.targetTextArea.value.substring(end);
                    this.targetTextArea.value = prev + next;
                    this.targetTextArea.selectionStart = start;
                    this.targetTextArea.selectionEnd = start;
                    break;
                }
            }
        }
        this.addToHistory(...actions);
        this.refreshContent();
    }

    load() {
        this.targetTextArea.value = this.loadCode();
        try {
            this.history = this.loadCodeHist();
        } catch (error) {
            if (error instanceof SyntaxError) {
                this.history = [];
                this.saveHistory();
            } else {
                throw error;
            }
        }
        this.refreshContent();
    }

    saveContent() {
        this.saveCode(this.targetTextArea.value);
    }

    saveHistory() {
        this.saveCodeHist(this.history);
    }

    highlight() {
        this.highlighter.highlight(this.targetTextArea, this.targetPre);
    }

    syncScroll() {
        this.targetPre.scrollTop = this.targetTextArea.scrollTop;
    }

    refreshPosition() {
        this.syncScroll();
        this.updateLineColumn();
    }

    refreshContent() {
        this.highlight();
        this.saveContent();
        this.refreshPosition();
    }

    updateLineColumn() {
        const position = this.targetTextArea.selectionStart;
        const prevText = this.targetTextArea.value.substring(0, position);
        let line = 1;
        for (const ch of prevText) {
            if (ch == '\n') {
                line++;
            }
        }
        const lineStart = prevText.lastIndexOf('\n') + 1;
        const column = position - lineStart + 1;

        this.currLineSpan.textContent = line;
        this.currColumnSpan.textContent = column;
    }

    isBetweenCurlies() {
        const start = this.targetTextArea.selectionStart;
        return (
            start > 0
            && this.targetTextArea.value[start - 1] == '{'
            && this.targetTextArea.value[start] == '}'
        );
    }

    isBetweenParens() {
        const start = this.targetTextArea.selectionStart;
        return (
            start > 0
            && this.targetTextArea.value[start - 1] == '('
            && this.targetTextArea.value[start] == ')'
        );
    }

    handleTab(evt) {
        evt.preventDefault();
        this.apply({ type: 'insert', data: '    ' });
    }

    handleEnter(evt) {
        if (this.isBetweenCurlies()) {
            evt.preventDefault();
            this.apply({ type: 'insert', data: '\n    \n' });
        }
    }

    handleBackspace(evt) {
        if (this.isBetweenCurlies() || this.isBetweenParens()) {
            evt.preventDefault();
            this.textAreaHTML.selectionStart--;
            this.textAreaHTML.selectionEnd++;
            this.apply({ type: 'delete' });
        }
    }


    handleParens(evt) {
        evt.preventDefault();
        this.apply({ type: 'insert', data: '()' });
    }

    handleCurly(evt) {
        evt.preventDefault();
        this.apply({ type: 'insert', data: '{}' });
    }

    handleKey(evt) {
        const keyMap = {
            'Tab': evt => this.handleTab(evt),
            'Enter': evt => this.handleEnter(evt),
            'Backspace': evt => this.handleBackspace(evt),
            '(': evt => this.handleParens(evt),
            '{': evt => this.handleCurly(evt)
        };

        if (evt.key in keyMap) {
            keyMap[evt.key](evt);
        }
    }
}

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
                    this.handleParens(
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

    handleParens(inputElement, piece, type, brackets, index, child) {
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

const editor = new Editor({
    targetTextArea: document.getElementById('userinput'),
    targetPre: document.getElementById('codeediting'),
    currLineSpan: document.getElementById('input-line'),
    currColumnSpan: document.getElementById('input-column'),
    saveCode,
    loadCode,
    saveCodeHist,
    loadCodeHist,
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
        editor.updateContent(evt.target.result);
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
