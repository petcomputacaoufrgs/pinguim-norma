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
        this.prevState = { selectionStart: 0, selectionEnd: 0, content: ''};
        this.history = { cursor: 0, entries: [] };
        this.historyLimit = params.historyLimit || 5000;

        this.refreshPrevState();

        this.targetTextArea.addEventListener('selectionchange', evt => {
            this.refreshContent();
        });

        this.targetTextArea.addEventListener('keydown', evt => {
            this.handleKey(evt);
        });

        this.targetTextArea.addEventListener('scroll', evt => {
            this.refreshPosition();
        });

        this.targetTextArea.addEventListener('input', evt => {
            this.handleUserEdit(evt);
        });

        this.targetTextArea.addEventListener('click', evt => {
            this.refreshContent();
        });
    }

    refreshPrevState() {
        this.prevState.selectionStart = this.targetTextArea.selectionStart;
        this.prevState.selectionEnd = this.targetTextArea.selectionEnd;
        this.prevState.content = this.targetTextArea.value;
    }

    updateContent(content) {
        this.targetTextArea.value = content;
        this.refreshContent();
    }

    get content() {
        return this.targetTextArea.value;
    }

    redo() {
        if (this.history.cursor < this.history.entries.length) {
            this.apply(this.history.entries[this.history.cursor]);
            this.history.cursor++;
        }
    }

    undo() {
        if (this.history.cursor > 0) {
            this.history.cursor--;
            this.applyRev(this.history.entries[this.history.cursor]);
        }
    }

    isHistoryValid() {
        if (typeof this.history != 'object' || this.history == null) {
            return false;
        }
        if (typeof this.history.cursor != 'number') {
            return false;
        }
        if (!(this.history.entries instanceof Array)) {
            return false;
        }
        for (const entry of this.history.entries) {
            if (typeof entry != 'object' || entry == null) {
                return false;
            }
            if (typeof entry.start != 'number') {
                return false;
            }
            if (typeof entry.oldText != 'string') {
                return false;
            }
            if (typeof entry.newText != 'string') {
                return false;
            }
        }
        return true;
    }

    addToHistory(action) {
        this.history.entries.splice(this.history.cursor);
        if (this.history.entries.length >= this.historyLimit) {
            const newStart = this.history.entries.length - this.historyLimit;
            this.history.entries.splice(0, newStart);
            this.history.cursor -= this.historyLimit;
        }
        this.history.entries.push(action);
        this.history.cursor++;
        this.saveHistory();
    }

    apply(action) {
        const end = action.start + action.oldText.length;
        const prev = this.targetTextArea.value.substring(0, action.start);
        const next = this.targetTextArea.value.substring(end);
        this.targetTextArea.value = prev + action.newText + next;

        const newPosition = action.start + action.newText.length;
        this.targetTextArea.selectionStart = newPosition;
        this.targetTextArea.selectionEnd = newPosition;
    }

    applyRev(action) {
        const end = action.start + action.newText.length;
        const prev = this.targetTextArea.value.substring(0, action.start);
        const next = this.targetTextArea.value.substring(end);
        this.targetTextArea.value = prev + action.oldText + next;

        const newPosition = action.start + action.oldText.length;
        this.targetTextArea.selectionStart = newPosition;
        this.targetTextArea.selectionEnd = newPosition;
    }

    edit(newText) {
        const start = this.targetTextArea.selectionStart;
        const end = this.targetTextArea.selectionEnd;
        const oldText = this.targetTextArea.value.substring(start, end);
        const action = { start, oldText, newText };

        this.apply(action);
        this.addToHistory(action);
        this.refreshContent();
    }

    load() {
        this.targetTextArea.value = this.loadCode();
        try {
            this.history = this.loadCodeHist();
            if (!this.isHistoryValid()) {
                this.resetHistory();
            }
        } catch (error) {
            if (error instanceof SyntaxError) {
                this.resetHistory();
            } else {
                throw error;
            }
        }
        this.refreshContent();
    }

    resetHistory() {
        this.history = { cursor: 0, entries: [] };
        this.saveHistory();
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
        this.refreshPrevState();
        this.highlight();
        this.refreshPosition();
        this.saveContent();
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

    handleUserEdit(evt) {
        evt.preventDefault();
        const start = (
            (this.prevState.selectionStart > this.targetTextArea.selectionStart)
            ? this.targetTextArea.selectionStart
            : this.prevState.selectionStart
        );
        const end = this.prevState.selectionEnd;
        const newEnd = this.targetTextArea.selectionEnd;
        const oldText = this.prevState.content.substring(start, end);
        const newText = this.targetTextArea.value.substring(start, newEnd);
        const action = { start, oldText, newText };

        this.addToHistory(action);
        this.refreshContent();
    }

    handleTab(evt) {
        evt.preventDefault();
        this.edit('    ');
    }

    handleEnter(evt) {
        if (this.isBetweenCurlies()) {
            evt.preventDefault();
            this.edit('\n    \n');
            this.targetTextArea.selectionStart--;
            this.targetTextArea.selectionEnd--;
        }
    }

    handleBackspace(evt) {
        if (this.isBetweenCurlies() || this.isBetweenParens()) {
            evt.preventDefault();
            this.targetTextArea.selectionStart--;
            this.targetTextArea.selectionEnd++;
            this.edit('');
        }
    }

    handleParens(evt) {
        evt.preventDefault();
        this.edit('()');
        this.targetTextArea.selectionStart--;
        this.targetTextArea.selectionEnd--;
    }

    handleCurly(evt) {
        evt.preventDefault();
        this.edit('{}');
        this.targetTextArea.selectionStart--;
        this.targetTextArea.selectionEnd--;
    }

    handleCtrlZ(evt) {
        evt.preventDefault();
        this.undo();
    }

    handleCtrlShiftZ(evt) {
        evt.preventDefault();
        this.redo();
    }

    handleCtrlY(evt) {
        evt.preventDefault();
        this.redo();
    }

    handleKey(evt) {
        const singleKeyMap = {
            'Tab': evt => this.handleTab(evt),
            'Enter': evt => this.handleEnter(evt),
            'Backspace': evt => this.handleBackspace(evt),
            '(': evt => this.handleParens(evt),
            '{': evt => this.handleCurly(evt)
        };

        const ctrlKeyMap = {
            'z': evt => this.handleCtrlZ(evt),
            'y': evt => this.handleCtrlY(evt),
        };

        const ctrlShiftKeyMap = {
            'Z': evt => this.handleCtrlShiftZ(evt),
        };

        if (evt.ctrlKey || evt.cmdKey) {
            if (evt.shiftKey) {
                if (evt.key in ctrlShiftKeyMap) {
                    ctrlShiftKeyMap[evt.key](evt);
                }
            } else if (evt.key in ctrlKeyMap) {
                ctrlKeyMap[evt.key](evt);
            }
        } else if (evt.key in singleKeyMap) {
            singleKeyMap[evt.key](evt);
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
