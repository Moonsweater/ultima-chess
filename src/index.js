import './styles/main.css';
import $ from 'jquery';
import './components/board.js';

jQuery(() => {
    console.log("jQuery loaded!");
})

document.addEventListener('DOMContentLoaded', () => {
    console.log("Dom loaded!");
})