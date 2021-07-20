// Theme
const toggleSwitch = document.querySelector('.theme-switch input[type="checkbox"]');
function switchTheme(e) {
    if (e.target.checked) {
        document.documentElement.setAttribute('data-theme', 'dark');
    }
    else {
        document.documentElement.setAttribute('data-theme', 'light');
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

/**
 * O que estiver dentro de // é expressão regular -> /conteudo/
 * O separador | indica 'ou' -> /a/ ou /b/ == /a|b/
 * A flag \b indica que a expressão precisa ser identica -> /\bIGUAL\b/
 * A flag 'g' significa global -> /tudoQueDarMatch/g
 * A flag 'i' significa case insensitive -> /AbCaTe/i == /ABACATE/i == /abacate/i
 */
const reservedWords = /\bmain\b|\bif\b|\bthen\b|\belse\b|\bdo\b|\bgoto\b|\boperation\b|\btest\b/gi;
const builtInFuncs = /\binc\b|\bdec\b|\bzero\b|\badd\b|\bsub\b|\bcmp\b/gi;
const regexLabels = /[0-9][:]/g;

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

// Tab key
// Allows tab identation inside textarea
textAreaHTML.addEventListener('keydown', (e) => {
    if(e.key == 'Tab') {
        e.preventDefault();
        const start = textAreaHTML.selectionStart;
        const end = textAreaHTML.selectionEnd;
        
        textAreaHTML.value = textAreaHTML.value.substring(0, start) + 
            "\t" + textAreaHTML.value.substring(end);

        textAreaHTML.selectionStart = textAreaHTML.selectionEnd = start + 1;
    }
});

// Bracket key
// Auto complete the bracket
textAreaHTML.addEventListener('keydown', (e) => {
    if(e.key == '(') {
        e.preventDefault();
        const start = textAreaHTML.selectionStart;
        const end = textAreaHTML.selectionEnd;

        textAreaHTML.value = textAreaHTML.value.substring(0, start) + 
            "()" + textAreaHTML.value.substring(end);

        textAreaHTML.selectionStart = textAreaHTML.selectionEnd = end + 1;
    }
});

// Curly bracket key
// Auto complete the curly bracket
textAreaHTML.addEventListener('keydown', (e) => {
    if(e.key == '{') {
        e.preventDefault();
        const start = textAreaHTML.selectionStart;
        const end = textAreaHTML.selectionEnd;

        textAreaHTML.value = textAreaHTML.value.substring(0, start) + 
            "{}" + textAreaHTML.value.substring(end);

        textAreaHTML.selectionStart = textAreaHTML.selectionEnd = end + 1;
    }
});

textAreaHTML.addEventListener('keydown', (e) => {
    if(e.key == 'Enter') {
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
});

