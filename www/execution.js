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
                const status = interpreter.runSteps(10000);
                running = status.running;

                if(running) {
                    setTimeout(tick, 10);
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

    //---------- TABELA CÓDIGO COMPILADO ==========  
    const tableCode = document.getElementById('table-compiled-program');

    const makeTable = () => {
        const instList = instructions();

        for(let i in instList) {
            const newRow = tableCode.insertRow();
            const pointerColumn = newRow.insertCell();
            const stepColumn = newRow.insertCell();
            const programColumn = newRow.insertCell();

            pointerColumn.classList.add("pointer_column");
            stepColumn.classList.add("step_column");
            programColumn.classList.add("program_column");

            stepColumn.innerHTML = instList[i]['label'];
            programColumn.innerHTML = instList[i]['kind'];
        }
    }
})
