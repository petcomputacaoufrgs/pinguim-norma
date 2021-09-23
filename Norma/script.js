// Local Storage
const setStorage = (userCode) => {
    localStorage.setItem("pinguim-norma-userCode", userCode);
}

const getStorage = () => {
    return localStorage.getItem("pinguim-norma-userCode");
}

const getLastCode = () => {
    textAreaHTML.innerHTML = getStorage();
    highlight();
}

window.onload = () => getLastCode();
window.onbeforeunload = () => setStorage(textAreaHTML.value);