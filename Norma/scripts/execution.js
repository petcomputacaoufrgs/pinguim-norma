// Control Buttons
const btnReset = document.getElementById("btn_reset");
const btnStep = document.getElementById("btn_step");
const btnRun = document.getElementById("btn_run");
const btnStop = document.getElementById("btn_stop");
const stepSpeed = document.getElementById("step_speed");
const displayWaitTime = document.getElementById("display_wait_time");

// TODO: Communication module[JS <---> WASM <---> Rust]
btnReset.onclick = () => {
    // TODO: limpar registradores e começar do inicio
}

btnStep.onclick = () => {
    // TODO: rodar apenas uma instrução
}

btnRun.onclick = () => {
    // TODO: rodar o programa na velocidade definida pelo usuario
}

btnStop.onclick = () => {
    // TODO: parar o programa sem resetar
}

stepSpeed.onchange = () => {
    displayWaitTime.innerHTML = "Espera entre passos (em ms): " + stepSpeed.value;
}