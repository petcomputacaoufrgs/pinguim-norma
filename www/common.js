import * as styles from './styles.css';
import * as commonStyles from './common_styles.css';
import * as wasm from "norma-wasm";

export function init(handler) {
    let initialized = false;

    window.addEventListener('DOMContentLoaded', () => {
        if (!initialized) {
            initialized = true;
            handler();
        }
    });

    if (document.readyState == 'complete' && !initialized) {
        initialized = true;
        handler();
    }
};

// Código para verificar se o wasm é suportado]
// Retirado de https://www.syncfusion.com/faq/how-can-i-check-if-a-browser-supports-webassembly
const supported = (() => {
    try {
        if (typeof WebAssembly === "object"
            && typeof WebAssembly.instantiate === "function")
        {
            const module = new WebAssembly.Module(Uint8Array.of(0x0, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00));
            if (module instanceof WebAssembly.Module)
                return new WebAssembly.Instance(module) instanceof WebAssembly.Instance;
        }
    } catch (e) { }
    return false;
})();
  
if (!supported) {
    alert("Seu navegador não suporta WebAssembly\nmude de navegador ou use a versão antiga");
}

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

const storageKey = "pinguim.norma.userCode";

// Local Storage
export const setStorage = (baseText) => {
    localStorage.setItem(storageKey, baseText);
};

export const getStorage = () => {
    return localStorage.getItem(storageKey);
};

// BEGIN Gambiarra pra testar WASM
init(() => {
    let interpreter = null;
    let running = false;

    const source = () => document.getElementById('userinput').value;
    const registerX = () => document.getElementById('gambiarra-reg-x').value;

    document.getElementById('gambiarra-check').onclick = () => {
        interpreter = null;
        try {
            console.log(wasm.check(source()));
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
        const tick = () => {
            if (running) {
                const status = interpreter.runSteps(10000);
                running = status.running;
                if (running) {
                    setTimeout(tick, 1);
                } else {
                    console.log(status);
                    console.log('Run All ended');
                }
            }
        };

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

