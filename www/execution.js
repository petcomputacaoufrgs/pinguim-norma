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
            console.log('wasm.compile ok');
            return true;
        }
        catch(error) {
            console.log('wasm.compile failed');
            return false;
        }
    }

    //---------- COMPILE TESTE ==========  
    const compileTest = () => {
        if(!compiled) {
            compile();
            setInput();
            compiled = true;
        }
    }

    //---------- INPUT REGISTRADOR X ========== 
    const setInput = () => {
        interpreter.input(registerX());
        console.log('interpreter.input ok');
    }

    //---------- RODAR PASSO ==========  
    document.getElementById('step').onclick = () => {
        compileTest();
        console.log(interpreter.runStep());    
    }

    //---------- RODAR N-PASSOS ========== 
    const runSteps = () => {
        compileTest();
        console.log(interpreter.runSteps(10000));
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
                    console.log(status);
                }
            }
            else {
                console.log('Ended run all...');
            }
        }
        running = true;
        interpreter.reset();
        interpreter.input(registerX());
        tick();
    }

    //---------- RESETAR CÓDIGO ========== 
    document.getElementById('reset').onclick = () => {
        compileTest();
        interpreter.reset();
    }

    //---------- ABORTAR PROGRAMA ==========  
    document.getElementById('abort').onclick = () => {
        compileTest();
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
})
