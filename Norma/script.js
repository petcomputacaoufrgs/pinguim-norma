// Theme
const toggleSwitch = document.querySelector('.theme-switch input[type="checkbox"]');
const toggleIcon = document.getElementById('toggle-icon');
function switchTheme(e) {
    if (e.target.checked) {
        document.documentElement.setAttribute('data-theme', 'dark');
        toggleIcon.innerHTML = 'dark_mode';
    }
    else {
        document.documentElement.setAttribute('data-theme', 'light');
        toggleIcon.innerHTML = 'light_mode';
    }    
}
toggleSwitch.addEventListener('change', switchTheme, false);

const currentTheme = localStorage.getItem('theme') ? localStorage.getItem('theme') : null;
if (currentTheme) {
    document.documentElement.setAttribute('data-theme', currentTheme);

    if (currentTheme === 'dark') {
        toggleSwitch.checked = true;
    }
}

// Upload and download buttons
const actualBtn = document.getElementById('upload_button');
const fileChosen = document.getElementById('file-chosen');
actualBtn.addEventListener('change', function(){
  fileChosen.textContent = this.files[0].name
})

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
    let finalText = baseText.replace(regexLabels, (match) => {return spanLabels + match + spanEnd});
    finalText = finalText.replace(reservedWords, (match) => {return spanReserved + match + spanEnd});
    finalText = finalText.replace(builtInFuncs, (match) => {return spanBuiltIn + match + spanEnd});

    codeAreaHTML.innerHTML = finalText
}

const handleKeys = {
    'Tab': (e) => {return handleTab(e)},
    'Enter': (e) => {return handleEnter(e)},
    'Backspace': (e) => {return handleBackspace(e)},
    '(': (e) => {return handleBracket(e)},
    '{': (e) => {return handleCurly(e)}
}

textAreaHTML.addEventListener('keydown', (e) => {
     try { handleKeys[e.key](e) }
     catch(e) {}
});

textAreaHTML.addEventListener('scroll', (e) => {
    handleScroll()
});

const handleScroll = () => {
    preAreaHTML.scrollTop = textAreaHTML.scrollTop;
    preAreaHTML.scrollLeft = textAreaHTML.scrollLeft;
}

const handleTab = (e) => {
    e.preventDefault();
    const start = textAreaHTML.selectionStart;
    const end = textAreaHTML.selectionEnd;

    textAreaHTML.value = textAreaHTML.value.substring(0, start) + 
        "\t" + textAreaHTML.value.substring(end);

    textAreaHTML.selectionStart = textAreaHTML.selectionEnd = start + 1;
}

const handleEnter = (e) => {
    const start = textAreaHTML.selectionStart;
    const end = textAreaHTML.selectionEnd;

    if((textAreaHTML.value[textAreaHTML.selectionStart - 1] == '{') && 
        (textAreaHTML.value[textAreaHTML.selectionStart] == '}')) {
        e.preventDefault();
        const start = textAreaHTML.selectionStart;
        const end = textAreaHTML.selectionEnd;

        textAreaHTML.value = textAreaHTML.value.substring(0, start) +
            "\n\t\n" + textAreaHTML.value.substring(end);

        textAreaHTML.selectionStart = textAreaHTML.selectionEnd = start + 2;
    }
}

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
}

const handleBracket = (e) => {
    e.preventDefault();
    const start = textAreaHTML.selectionStart;
    const end = textAreaHTML.selectionEnd;

    textAreaHTML.value = textAreaHTML.value.substring(0, start) + 
        "()" + textAreaHTML.value.substring(end);

    textAreaHTML.selectionStart = textAreaHTML.selectionEnd = end + 1;
}

const handleCurly = (e) => {
    e.preventDefault();
    const start = textAreaHTML.selectionStart;
    const end = textAreaHTML.selectionEnd;

    textAreaHTML.value = textAreaHTML.value.substring(0, start) + 
        "{}" + textAreaHTML.value.substring(end);

    textAreaHTML.selectionStart = textAreaHTML.selectionEnd = end + 1;
}