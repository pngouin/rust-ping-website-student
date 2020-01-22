import * as wasm from 'client';

const SERVER_URL = 'http://localhost:8000'

const pingRequestForm = document.forms['ping-request-form'];
const resultContainer = document.getElementById('result-container');

const displayResult = (domain, iterations, ping) => {
    const paragraph = document.createElement('p');
    document.textContent = `${domain} :: ${iterations}ms (avg. on ${ping})`;
    resultContainer.appendChild(paragraph);
}

const pingServer = async (ev) => {
    ev.preventDefault();

    const inputs = pingRequestForm.getElementsByTagName('input');
    const domain = encodeURIComponent(inputs[0].value);
    const iterations = encodeURIComponent(inputs[1].value);
    const body = `domain=${domain}&iterations=${iterations}`;

    const url = `${SERVER_URL}/ping?${body}`;

    const xhr = new XMLHttpRequest();
    xhr.open('POST', url, true);
    
    xhr.setRequestHeader('Content-Type', 'application/x-www-form-urlencoded');

    xhr.onreadystatechange = function() {
        console.log(this.readyState, this.status, this.response, this.responseText);
        // if (this.readyState === 4 && this.status === 200) {
        //     console.log(this.response, this.responseText);
        // }
    }

    xhr.send(body);

    return false;
}

pingRequestForm.addEventListener('submit', pingServer);