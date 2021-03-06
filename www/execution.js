import { init, loadCode } from './common.js';
import * as wasm from "norma-wasm";

init(() => {
    let interpreter = null;
    let running = false;
    let compiled = false;
    let stepSpeed = 0;

    const source = () => loadCode();
    const userInput = document.getElementById('input');
    const registerX = () => document.getElementById('input').value;

    const reset = () => {
        interpreter.reset();
        setInput();
        cleanHTML();
        running = false;
    };

    //---------- ATUALIZA X ON CHANGE ==========
    userInput.onchange = () => {
        reset();
        document.getElementById('reg-value-X').innerHTML = userInput.value
    }

    //---------- COMPILAR CÓDIGO  ==========
    const compile = () => {
        interpreter = null;
        try {
            interpreter = wasm.compile(source());
            return true;
        }
        catch(error) {
            return false;
        }
    }

    //---------- COMPILE TESTE ==========
    const compileTest = () => {
        if (!compiled) {
            compile();
            setInput();
            compiled = true;
            makeTable();
            makeRegisters();
        }
    }

    //---------- INPUT REGISTRADOR X ==========
    const setInput = () => {
        interpreter.input(registerX());
    }

    //---------- RODAR PASSO ==========
    document.getElementById('step').onclick = () => {
        compileTest();
        const status = interpreter.runSteps(1);
        running = status.running;
        updateRegisters();

        let line = data();
        lineHighlight(line['status']['currentLabel']);
    }

    //---------- UPDATE REGISTERS ==========
    const updateRegisters = () => {
        let registers = data();
        let numPassos = registers['status']['steps'];
        registers = registers['status']['registers'];

        for(let i in registers) {
            document.getElementById('reg-value-' + registers[i]['name']).innerHTML = registers[i]['value'];
            updatePassos(numPassos);
        }
        updateSaida();
    }

    //---------- RODAR N-PASSOS ==========
    const runSteps = () => {
        compileTest();
        interpreter.runSteps(10000);
    }

    //---------- RODAR TODOS PASSOS ==========
    document.getElementById('run').onclick = () => {
        compileTest();

        const tick = () => {
            if (running) {
                const status = interpreter.runSteps(stepSpeed ? 1 : 1347);
                running = status.running;
                updateRegisters();

                if (running) {
                    lineHighlight(status.currentLabel);
                    setTimeout(tick, stepSpeed || 1);
                } else {
                    const end = performance.now();
                }
            }
        }

        running = true;
        const then = performance.now();
        tick();
    }

    //---------- RESETAR CÓDIGO ==========
    document.getElementById('reset').onclick = () => {
        reset();
    }

    //---------- ABORTAR PROGRAMA ==========
    document.getElementById('abort').onclick = () => {
        running = false;
    }

    //---------- DADOS DO CÓDIGO ==========
    const data = () => {
        compileTest();
        return interpreter.data();
    }

    //---------- INSTRUÇÕES DO CÓDIGO ==========
    const instructions = () => {
        compileTest();
        return interpreter.instructions();
    }

    //---------- STATUS DO CÓDIGO ==========
    const codeStatus = () => {
        compileTest();
        return interpreter.status();
    }

    //---------- STEP SPEED CONTROL ==========
    const stepHeader = document.getElementById('step-header');
    const stepControl = document.getElementById('step-control');

    const renderStepControl = () => {
        stepSpeed = parseInt(stepControl.value);
        if (stepSpeed == 0) {
            stepHeader.innerHTML = 'sem espera';
        } else {
            stepHeader.innerHTML = stepSpeed + ' (ms)';
        }
    };

    renderStepControl();

    document.getElementById('step-control').onchange = () => {
        stepSpeed = stepControl.value;
        renderStepControl();
    }

    //---------- REGISTRADORES NO HTML ==========
    const regSection = document.getElementById('registers-section');

    const makeRegisters = () => {
        let registers = data();
        registers = registers['status']['registers'];

        for(let i in registers) {
            const outerDiv = document.createElement('div');
            outerDiv.id = 'reg-' + registers[i]['name'];
            outerDiv.className = 'register';
            regSection.appendChild(outerDiv);

            const innerH3 = document.createElement('h3');
            const innerDiv = document.createElement('div');
            innerH3.id = 'reg-name-' + registers[i]['name'];
            innerDiv.id = 'reg-value-' + registers[i]['name'];
            innerH3.className = 'register_name';
            innerDiv.className = 'register_value';
            innerH3.innerText = registers[i]['name'];
            innerDiv.innerText = registers[i]['value'];
            outerDiv.appendChild(innerH3);
            outerDiv.appendChild(innerDiv);
        }
    }

    //---------- TABELA CÓDIGO COMPILADO ==========
    const tableCode = document.getElementById('table-compiled-program');

    const makeTable = () => {
        const instList = instructions();

        for(let i in instList) {
            const newRow = tableCode.insertRow();
            const stepColumn = newRow.insertCell();
            const programColumn = newRow.insertCell();

            stepColumn.classList.add("step_column");
            programColumn.classList.add("program_column");

            stepColumn.innerHTML = instList[i]['label'];
            programColumn.innerHTML = instList[i]['kind'];
            newRow.id = instList[i]['label'];
        }
    }

    //---------- COMPILAR AO CARREGAR ==========
    compileTest()

    //---------- HIGHLIGHT LINHA ATUAL ==========
    let lastLine;
    if (tableCode.firstElementChild != undefined) {
        lastLine = tableCode.firstElementChild.firstChild;
    }
    if (lastLine) {
        lastLine.classList.add('line_selected');
    }
    let firstLine = lastLine;
    const lineHighlight = (lineId) => {
        try {
            if (lastLine) {
                lastLine.classList.remove('line_selected')
            }

            let actualLine = document.getElementById(lineId);
            actualLine.classList.add('line_selected');
            lastLine = actualLine;
       } catch(e) {}
    }

    //---------- UPDATE NÚMERO DE PASSOS ==========
    const numPassos = document.getElementById('num-passos');
    const updatePassos = (num) => numPassos.innerHTML = num;

    //---------- UPDATE SAÍDA ==========
    const outputSpan = document.getElementById('saida');

    const updateSaida = () => {
        if(running) {
            outputSpan.innerHTML = 'Rodando...';
        } else {
            let values = data();
            values = values.status.registers.find(values => values.name == "Y");
            outputSpan.innerHTML = values.value;
        }
    }

    //---------- LIMPA HTML QUANDO RESETAR ==========
    const cleanHTML = () => {
        numPassos.innerHTML = '0';
        outputSpan.innerHTML = '';
        if (lastLine) {
            lastLine.classList.remove('line_selected');
            firstLine.classList.add('line_selected');
            lastLine = firstLine;
        }

        for (let i in regSection.children) {
            try {
                regSection.children[i].lastChild.innerHTML = 0
            } catch(e) {}
        }
        document.getElementById('reg-value-X').innerHTML = userInput.value
    }
})
