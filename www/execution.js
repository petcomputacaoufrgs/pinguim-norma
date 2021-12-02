import { init, getStorage } from './common.js';
import * as wasm from "norma-wasm";

init(() => {
    let interpreter = null;
    let running = false;
    let compiled = false;

    const source = () => getStorage();
    const registerX = () => document.getElementById('input').value;

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
        if(!compiled) {
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
        interpreter.runStep();    

        updateRegisters();
    }

    //---------- UPDATE REGISTERS ==========  
    const updateRegisters = () => {
        let registers = data();
        registers = registers['status']['registers'];

        for(let i in registers) {
            document.getElementById('reg-value-' + registers[i]['name']).innerHTML = registers[i]['value'];
        }
    }

    //---------- RODAR N-PASSOS ========== 
    const runSteps = () => {
        compileTest();
        interpreter.runSteps(10000);
    }

    //---------- RODAR TODOS PASSOS ==========  
    document.getElementById('run').onclick = () => {
        const then = performance.now();

        compileTest();

        const tick = () => {
            if(running) {
                const status = interpreter.runSteps(1);
                running = status.running;

                if(running) {
                    updateRegisters();
                    setTimeout(tick, stepSpeed);
                }
                else {
                    const end = performance.now();
                }
            }
        }
        running = true;
        interpreter.reset();
        interpreter.input(registerX());
        tick();
    }

    //---------- RESETAR CÓDIGO ========== 
    document.getElementById('reset').onclick = () => {
        interpreter.reset();
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
    let stepSpeed = 1000;

    document.getElementById('step-control').onchange = () => {
        stepHeader.innerHTML = stepControl.value;
        stepSpeed = stepControl.value;
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
        }
    }
})

/*
operation clear(A) {
    1: if zero A then goto 0 else goto 2
    2: do dec A goto 1
}
*/
