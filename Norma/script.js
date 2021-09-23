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